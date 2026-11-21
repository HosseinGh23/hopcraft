use std::collections::HashMap;

use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;
use serde_json::{json, Map, Value};
use url::Url;

use crate::error::AppError;

#[derive(Debug, Clone, Serialize)]
pub struct LinkMeta {
    pub protocol: String,
    pub address: String,
    pub port: u16,
}

pub fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hi = (bytes[i + 1] as char).to_digit(16);
            let lo = (bytes[i + 2] as char).to_digit(16);
            if let (Some(h), Some(l)) = (hi, lo) {
                out.push(((h << 4) | l) as u8);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

pub fn decode_b64_flexible(raw: &str) -> Result<String, ()> {
    let decoded_pct = percent_decode(raw.trim());
    let cleaned: String = decoded_pct
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    let unpadded = cleaned.trim_end_matches('=');

    let bytes = general_purpose::URL_SAFE_NO_PAD
        .decode(unpadded)
        .or_else(|_| general_purpose::STANDARD_NO_PAD.decode(unpadded))
        .map_err(|_| ())?;

    String::from_utf8(bytes).map_err(|_| ())
}

fn clean_host(host: &str) -> String {
    host.trim_start_matches('[')
        .trim_end_matches(']')
        .to_string()
}

fn sample_preview(uri: &str) -> String {
    let truncated: String = uri.chars().take(40).collect();
    format!("{}...", truncated)
}

fn build_stream_settings(
    network_in: &str,
    security_in: &str,
    params: &HashMap<String, String>,
    fallback_host: &str,
) -> Value {
    let network = if network_in.is_empty() {
        "tcp".to_string()
    } else {
        network_in.to_lowercase()
    };
    let security = if security_in.is_empty() {
        "none".to_string()
    } else {
        security_in.to_lowercase()
    };

    let host = params.get("host").map(|s| s.as_str()).unwrap_or("");
    let path = params
        .get("path")
        .map(|s| s.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or("/");
    let sni = params.get("sni").map(|s| s.as_str()).unwrap_or("");
    let fp = params.get("fp").map(|s| s.as_str()).unwrap_or("");
    let alpn = params.get("alpn").map(|s| s.as_str()).unwrap_or("");

    let mut stream = Map::new();
    stream.insert("network".to_string(), Value::String(network.clone()));
    stream.insert("security".to_string(), Value::String(security.clone()));

    match network.as_str() {
        "ws" => {
            let mut ws_cfg = Map::new();
            ws_cfg.insert("path".to_string(), Value::String(path.to_string()));
            if !host.is_empty() {
                ws_cfg.insert("headers".to_string(), json!({ "Host": host }));
            }
            stream.insert("wsSettings".to_string(), Value::Object(ws_cfg));
        }
        "grpc" => {
            let svc = params
                .get("serviceName")
                .map(|s| s.as_str())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| path.trim_start_matches('/'));
            let mut grpc_cfg = Map::new();
            grpc_cfg.insert("serviceName".to_string(), Value::String(svc.to_string()));
            if params.get("mode").map(|s| s.as_str()) == Some("multi") {
                grpc_cfg.insert("multiMode".to_string(), Value::Bool(true));
            }
            stream.insert("grpcSettings".to_string(), Value::Object(grpc_cfg));
        }
        "tcp" => {
            if params.get("headerType").map(|s| s.as_str()) == Some("http") {
                let mut req = Map::new();
                req.insert("path".to_string(), json!([path]));
                if !host.is_empty() {
                    req.insert("headers".to_string(), json!({ "Host": [host] }));
                }
                stream.insert(
                    "tcpSettings".to_string(),
                    json!({ "header": { "type": "http", "request": Value::Object(req) } }),
                );
            }
        }
        "xhttp" | "splithttp" => {
            let key = if network == "xhttp" {
                "xhttpSettings"
            } else {
                "splithttpSettings"
            };
            let mut cfg = Map::new();
            cfg.insert("path".to_string(), Value::String(path.to_string()));
            if !host.is_empty() {
                cfg.insert("host".to_string(), Value::String(host.to_string()));
            }
            stream.insert(key.to_string(), Value::Object(cfg));
        }
        "httpupgrade" => {
            let mut cfg = Map::new();
            cfg.insert("path".to_string(), Value::String(path.to_string()));
            if !host.is_empty() {
                cfg.insert("host".to_string(), Value::String(host.to_string()));
            }
            stream.insert("httpupgradeSettings".to_string(), Value::Object(cfg));
        }
        "h2" | "http" => {
            let mut cfg = Map::new();
            cfg.insert("path".to_string(), Value::String(path.to_string()));
            if !host.is_empty() {
                cfg.insert("host".to_string(), json!([host]));
            }
            stream.insert("httpSettings".to_string(), Value::Object(cfg));
        }
        _ => {}
    }

    let effective_sni = if !sni.is_empty() {
        sni
    } else if !host.is_empty() {
        host
    } else {
        fallback_host
    };

    if security == "tls" {
        let allow_insecure = params
            .get("allowInsecure")
            .map(|v| matches!(v.to_lowercase().as_str(), "1" | "true"))
            .unwrap_or(false);
        let mut tls_cfg = Map::new();
        tls_cfg.insert(
            "serverName".to_string(),
            Value::String(effective_sni.to_string()),
        );
        tls_cfg.insert("allowInsecure".to_string(), Value::Bool(allow_insecure));
        if !fp.is_empty() {
            tls_cfg.insert("fingerprint".to_string(), Value::String(fp.to_string()));
        }
        if !alpn.is_empty() {
            let list: Vec<Value> = alpn
                .split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| Value::String(s.to_string()))
                .collect();
            if !list.is_empty() {
                tls_cfg.insert("alpn".to_string(), Value::Array(list));
            }
        }
        stream.insert("tlsSettings".to_string(), Value::Object(tls_cfg));
    } else if security == "reality" {
        let pbk = params.get("pbk").map(|s| s.as_str()).unwrap_or("");
        let sid = params.get("sid").map(|s| s.as_str()).unwrap_or("");
        let spx = params
            .get("spx")
            .map(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("/");
        let fingerprint = if fp.is_empty() { "chrome" } else { fp };
        stream.insert(
            "realitySettings".to_string(),
            json!({
                "serverName": effective_sni,
                "fingerprint": fingerprint,
                "publicKey": pbk,
                "shortId": sid,
                "spiderX": spx
            }),
        );
    }

    Value::Object(stream)
}

fn parse_query_map(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in query.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (k, v) = match pair.split_once('=') {
            Some((k, v)) => (k, v),
            None => (pair, ""),
        };
        let key = percent_decode(k);
        let val = percent_decode(&v.replace('+', " "));
        map.entry(key).or_insert(val);
    }
    map
}

fn parse_shadowsocks_link(uri: &str, tag: &str) -> Result<(Value, LinkMeta), AppError> {
    let body = uri["ss://".len()..].trim();
    let body = body.split('#').next().unwrap_or("");
    let (main_part_raw, query_str) = match body.split_once('?') {
        Some((m, q)) => (m, q),
        None => (body, ""),
    };
    let main_part = main_part_raw.trim_end_matches('/');
    let mut params = parse_query_map(query_str);

    let (method, password, hostport) = if let Some((userinfo_raw, hp)) = main_part.rsplit_once('@')
    {
        let userinfo_unquoted = percent_decode(userinfo_raw);
        if let Some((m, p)) = userinfo_unquoted.split_once(':') {
            (m.to_string(), p.to_string(), hp.to_string())
        } else {
            let decoded = decode_b64_flexible(userinfo_raw).map_err(|_| {
                AppError::new("err_invalid_ss_b64", "Invalid Base64 in Shadowsocks link.")
            })?;
            let (m, p) = decoded.split_once(':').ok_or_else(|| {
                AppError::new(
                    "err_invalid_ss_auth",
                    "Invalid method:password in Shadowsocks link.",
                )
            })?;
            (m.to_string(), p.to_string(), hp.to_string())
        }
    } else {
        let decoded = decode_b64_flexible(main_part).map_err(|_| {
            AppError::new("err_invalid_ss_b64", "Invalid Base64 in Shadowsocks link.")
        })?;
        let (userinfo, hp) = decoded
            .rsplit_once('@')
            .ok_or_else(|| AppError::new("err_invalid_ss", "Invalid Shadowsocks link format."))?;
        let (m, p) = userinfo.split_once(':').ok_or_else(|| {
            AppError::new(
                "err_invalid_ss_auth",
                "Invalid method:password in Shadowsocks link.",
            )
        })?;
        (m.to_string(), p.to_string(), hp.to_string())
    };

    let parsed_hp = Url::parse(&format!("ss://{}", hostport))
        .map_err(|_| AppError::new("err_incomplete_ss", "Incomplete Shadowsocks link fields."))?;
    let address = parsed_hp
        .host_str()
        .map(clean_host)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| AppError::new("err_incomplete_ss", "Incomplete Shadowsocks link fields."))?;
    let port = parsed_hp
        .port()
        .ok_or_else(|| AppError::new("err_incomplete_ss", "Incomplete Shadowsocks link fields."))?;

    if method.is_empty() || password.is_empty() {
        return Err(AppError::new(
            "err_incomplete_ss",
            "Incomplete Shadowsocks link fields.",
        ));
    }

    let mut network = params
        .get("type")
        .cloned()
        .unwrap_or_else(|| "tcp".to_string());
    let mut security = params
        .get("security")
        .cloned()
        .unwrap_or_else(|| "none".to_string());

    if let Some(plugin_str) = params.get("plugin").cloned() {
        let parts: Vec<&str> = plugin_str
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        if let Some(first) = parts.first() {
            if first.to_lowercase().contains("v2ray-plugin") {
                network = "ws".to_string();
                for item in parts.iter().skip(1) {
                    if let Some((k, v)) = item.split_once('=') {
                        let kl = k.trim().to_lowercase();
                        let vt = v.trim();
                        if matches!(kl.as_str(), "host" | "path" | "sni") {
                            params.insert(kl, vt.to_string());
                        } else if kl == "mode" && vt.eq_ignore_ascii_case("websocket") {
                            network = "ws".to_string();
                        }
                    } else if item.eq_ignore_ascii_case("tls") {
                        security = "tls".to_string();
                    }
                }
            }
        }
    }

    let stream_settings = build_stream_settings(&network, &security, &params, &address);
    let outbound = json!({
        "tag": tag,
        "protocol": "shadowsocks",
        "settings": {
            "servers": [
                {
                    "address": address,
                    "port": port,
                    "method": method,
                    "password": password
                }
            ]
        },
        "streamSettings": stream_settings
    });

    let meta = LinkMeta {
        protocol: "SS".to_string(),
        address,
        port,
    };
    Ok((outbound, meta))
}

fn value_as_string(v: Option<&Value>) -> String {
    match v {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        _ => String::new(),
    }
}

fn value_as_u16(v: Option<&Value>) -> Option<u16> {
    match v {
        Some(Value::Number(n)) => n.as_u64().and_then(|x| u16::try_from(x).ok()),
        Some(Value::String(s)) => s.trim().parse::<u16>().ok(),
        _ => None,
    }
}

pub fn parse_v2ray_link(uri_raw: &str, tag: &str) -> Result<(Value, LinkMeta), AppError> {
    let uri = uri_raw.trim();
    if uri.is_empty() {
        return Err(AppError::new("err_empty_link", "Proxy link is empty."));
    }

    if uri.starts_with("vless://") {
        let parsed = Url::parse(uri).map_err(|_| {
            AppError::with_sample(
                "err_invalid_vless",
                sample_preview(uri),
                "Invalid VLESS URI format.",
            )
        })?;

        let user_id = percent_decode(parsed.username());
        let address = parsed.host_str().map(clean_host).unwrap_or_default();
        let port = parsed.port();

        if user_id.is_empty() || address.is_empty() || port.is_none() {
            return Err(AppError::with_sample(
                "err_invalid_vless",
                sample_preview(uri),
                "Invalid VLESS URI format.",
            ));
        }
        let port = port.unwrap();

        let mut params = HashMap::new();
        for (k, v) in parsed.query_pairs() {
            params.entry(k.into_owned()).or_insert_with(|| v.into_owned());
        }

        let encryption = params
            .get("encryption")
            .filter(|s| !s.is_empty())
            .cloned()
            .unwrap_or_else(|| "none".to_string());

        let mut user = Map::new();
        user.insert("id".to_string(), Value::String(user_id));
        user.insert("encryption".to_string(), Value::String(encryption));
        if let Some(flow) = params.get("flow").filter(|s| !s.is_empty()) {
            user.insert("flow".to_string(), Value::String(flow.clone()));
        }

        let net = params.get("type").map(|s| s.as_str()).unwrap_or("tcp");
        let sec = params.get("security").map(|s| s.as_str()).unwrap_or("none");
        let stream_settings = build_stream_settings(net, sec, &params, &address);

        let outbound = json!({
            "tag": tag,
            "protocol": "vless",
            "settings": {
                "vnext": [
                    {
                        "address": address,
                        "port": port,
                        "users": [Value::Object(user)]
                    }
                ]
            },
            "streamSettings": stream_settings
        });

        let meta = LinkMeta {
            protocol: "VLESS".to_string(),
            address,
            port,
        };
        return Ok((outbound, meta));
    }

    if uri.starts_with("trojan://") {
        let parsed = Url::parse(uri).map_err(|_| {
            AppError::with_sample(
                "err_invalid_trojan",
                sample_preview(uri),
                "Invalid Trojan URI format.",
            )
        })?;

        let password = percent_decode(parsed.username());
        let address = parsed.host_str().map(clean_host).unwrap_or_default();
        let port = parsed.port();

        if password.is_empty() || address.is_empty() || port.is_none() {
            return Err(AppError::with_sample(
                "err_invalid_trojan",
                sample_preview(uri),
                "Invalid Trojan URI format.",
            ));
        }
        let port = port.unwrap();

        let mut params = HashMap::new();
        for (k, v) in parsed.query_pairs() {
            params.entry(k.into_owned()).or_insert_with(|| v.into_owned());
        }

        let net = params.get("type").map(|s| s.as_str()).unwrap_or("tcp");
        let sec = params
            .get("security")
            .map(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("tls");
        let stream_settings = build_stream_settings(net, sec, &params, &address);

        let outbound = json!({
            "tag": tag,
            "protocol": "trojan",
            "settings": {
                "servers": [
                    {
                        "address": address,
                        "port": port,
                        "password": password
                    }
                ]
            },
            "streamSettings": stream_settings
        });

        let meta = LinkMeta {
            protocol: "Trojan".to_string(),
            address,
            port,
        };
        return Ok((outbound, meta));
    }

    if uri.starts_with("vmess://") {
        let raw = uri["vmess://".len()..].trim();
        let decoded = decode_b64_flexible(raw).map_err(|_| {
            AppError::new("err_invalid_vmess_b64", "Invalid Base64 in VMess link.")
        })?;
        let data: Value = serde_json::from_str(&decoded).map_err(|_| {
            AppError::new("err_invalid_vmess_b64", "Invalid JSON payload in VMess link.")
        })?;

        let address = value_as_string(data.get("add"));
        let port = value_as_u16(data.get("port")).unwrap_or(0);
        let id = value_as_string(data.get("id"));
        if address.is_empty() || port == 0 || id.is_empty() {
            return Err(AppError::new(
                "err_incomplete_vmess",
                "Incomplete VMess link fields.",
            ));
        }

        let aid = value_as_u16(data.get("aid")).unwrap_or(0);
        let scy_raw = value_as_string(data.get("scy"));
        let scy = if scy_raw.is_empty() {
            "auto".to_string()
        } else {
            scy_raw
        };

        let mut params = HashMap::new();
        params.insert("host".to_string(), value_as_string(data.get("host")));
        let path_val = value_as_string(data.get("path"));
        params.insert(
            "path".to_string(),
            if path_val.is_empty() {
                "/".to_string()
            } else {
                path_val
            },
        );
        params.insert("sni".to_string(), value_as_string(data.get("sni")));
        params.insert("fp".to_string(), value_as_string(data.get("fp")));
        params.insert("alpn".to_string(), value_as_string(data.get("alpn")));
        params.insert("headerType".to_string(), value_as_string(data.get("type")));

        let tls_flag = value_as_string(data.get("tls"));
        let security = if tls_flag.eq_ignore_ascii_case("tls") {
            "tls"
        } else {
            "none"
        };
        let net_raw = value_as_string(data.get("net"));
        let net = if net_raw.is_empty() { "tcp" } else { &net_raw };

        let stream_settings = build_stream_settings(net, security, &params, &address);
        let outbound = json!({
            "tag": tag,
            "protocol": "vmess",
            "settings": {
                "vnext": [
                    {
                        "address": address,
                        "port": port,
                        "users": [
                            {
                                "id": id,
                                "alterId": aid,
                                "security": scy
                            }
                        ]
                    }
                ]
            },
            "streamSettings": stream_settings
        });

        let meta = LinkMeta {
            protocol: "VMess".to_string(),
            address,
            port,
        };
        return Ok((outbound, meta));
    }

    if uri.starts_with("ss://") {
        return parse_shadowsocks_link(uri, tag);
    }

    Err(AppError::new(
        "err_unsupported_protocol",
        "Unsupported protocol. Use vless://, vmess://, trojan://, or ss://.",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_vless_ws() {
        let uri = "vless://a3cf12da-e530-11f1-ba64-07072239f532@31.169.127.226:10004?encryption=none&security=none&type=ws&path=%2Fvpnjantit#hop1";
        let (out, meta) = parse_v2ray_link(uri, "proxy-entry").unwrap();
        assert_eq!(meta.protocol, "VLESS");
        assert_eq!(meta.address, "31.169.127.226");
        assert_eq!(meta.port, 10004);
        assert_eq!(out["streamSettings"]["wsSettings"]["path"], "/vpnjantit");
    }

    #[test]
    fn parses_shadowsocks_sip002() {
        let uri = "ss://YWVzLTI1Ni1nY206cGFzc3dvcmQ=@1.2.3.4:8388#test";
        let (out, meta) = parse_v2ray_link(uri, "proxy-exit").unwrap();
        assert_eq!(meta.protocol, "SS");
        assert_eq!(meta.address, "1.2.3.4");
        assert_eq!(out["settings"]["servers"][0]["method"], "aes-256-gcm");
        assert_eq!(out["settings"]["servers"][0]["password"], "password");
    }
}
