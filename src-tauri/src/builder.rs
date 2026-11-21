use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::AppError;
use crate::parser::{parse_v2ray_link, LinkMeta};

#[derive(Debug, Deserialize)]
pub struct ChainRequest {
    pub hop1: String,
    pub hop2: String,
    pub listen: String,
    pub mixed_port: Option<u16>,
    pub socks_port: Option<u16>,
    pub http_port: Option<u16>,
}

#[derive(Debug, Serialize)]
pub struct ChainResponse {
    pub json: String,
    pub lines: usize,
    pub bytes: usize,
    pub entry: LinkMeta,
    pub exit: LinkMeta,
}

pub fn build_chain_config(req: &ChainRequest) -> Result<ChainResponse, AppError> {
    let (entry_out, entry_meta) = parse_v2ray_link(&req.hop1, "proxy-entry")?;
    let (mut exit_out, exit_meta) = parse_v2ray_link(&req.hop2, "proxy-exit")?;

    // Chain Hop 2 through Hop 1 so outgoing IP always belongs to Hop 2
    if let Some(obj) = exit_out.as_object_mut() {
        let stream = obj
            .entry("streamSettings".to_string())
            .or_insert_with(|| json!({}));
        if let Some(stream_obj) = stream.as_object_mut() {
            stream_obj.insert(
                "sockopt".to_string(),
                json!({ "dialerProxy": "proxy-entry" }),
            );
        }
    }

    let listen = if req.listen.trim().is_empty() {
        "127.0.0.1"
    } else {
        req.listen.trim()
    };

    let effective_mixed = match (req.mixed_port, req.socks_port, req.http_port) {
        (Some(p), _, _) => Some(p),
        (None, Some(sp), Some(hp)) if sp == hp => Some(sp),
        _ => None,
    };

    let mut inbounds: Vec<Value> = Vec::new();
    let mut inbound_tags: Vec<String> = Vec::new();

    if let Some(port) = effective_mixed {
        inbounds.push(json!({
            "tag": "mixed-in",
            "listen": listen,
            "port": port,
            "protocol": "mixed",
            "settings": { "auth": "noauth", "udp": true },
            "sniffing": {
                "enabled": true,
                "destOverride": ["http", "tls", "quic"]
            }
        }));
        inbound_tags.push("mixed-in".to_string());
    } else {
        if req.socks_port.is_none() && req.http_port.is_none() {
            return Err(AppError::new(
                "err_no_inbound",
                "At least one inbound port (Mixed, SOCKS5, or HTTP) must be enabled.",
            ));
        }

        if let Some(port) = req.socks_port {
            inbounds.push(json!({
                "tag": "socks-in",
                "listen": listen,
                "port": port,
                "protocol": "socks",
                "settings": { "auth": "noauth", "udp": true },
                "sniffing": {
                    "enabled": true,
                    "destOverride": ["http", "tls", "quic"]
                }
            }));
            inbound_tags.push("socks-in".to_string());
        }

        if let Some(port) = req.http_port {
            inbounds.push(json!({
                "tag": "http-in",
                "listen": listen,
                "port": port,
                "protocol": "http",
                "settings": {},
                "sniffing": {
                    "enabled": true,
                    "destOverride": ["http", "tls", "quic"]
                }
            }));
            inbound_tags.push("http-in".to_string());
        }
    }

    let config = json!({
        "log": { "loglevel": "warning" },
        "inbounds": inbounds,
        "outbounds": [
            exit_out,
            entry_out,
            { "tag": "direct", "protocol": "freedom" },
            { "tag": "block", "protocol": "blackhole" }
        ],
        "routing": {
            "domainStrategy": "AsIs",
            "rules": [
                {
                    "type": "field",
                    "ip": [
                        "127.0.0.0/8",
                        "10.0.0.0/8",
                        "172.16.0.0/12",
                        "192.168.0.0/16",
                        "::1/128"
                    ],
                    "outboundTag": "direct"
                },
                {
                    "type": "field",
                    "inboundTag": inbound_tags,
                    "outboundTag": "proxy-exit"
                }
            ]
        }
    });

    let pretty = serde_json::to_string_pretty(&config)
        .map_err(|e| AppError::new("err_serialize", e.to_string()))?;
    let lines = pretty.lines().count();
    let bytes = pretty.len();

    Ok(ChainResponse {
        json: pretty,
        lines,
        bytes,
        entry: entry_meta,
        exit: exit_meta,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_chained_mixed_config() {
        let req = ChainRequest {
            hop1: "vless://a3cf12da-e530-11f1-ba64-07072239f532@31.169.127.226:10004?encryption=none&security=none&type=ws&path=%2Fvpnjantit#hop1".to_string(),
            hop2: "vless://c9aa7184-e472-11f1-b598-2f49e4fd77f8@212.192.28.46:10004?encryption=none&security=none&type=ws&path=%2Fvpnjantit#hop2".to_string(),
            listen: "127.0.0.1".to_string(),
            mixed_port: Some(10808),
            socks_port: None,
            http_port: None,
        };
        let resp = build_chain_config(&req).unwrap();
        let parsed: Value = serde_json::from_str(&resp.json).unwrap();
        assert_eq!(parsed["inbounds"][0]["protocol"], "mixed");
        assert_eq!(parsed["inbounds"][0]["port"], 10808);
        assert_eq!(parsed["outbounds"][0]["tag"], "proxy-exit");
        assert_eq!(
            parsed["outbounds"][0]["streamSettings"]["sockopt"]["dialerProxy"],
            "proxy-entry"
        );
        assert_eq!(resp.exit.address, "212.192.28.46");
    }
}
