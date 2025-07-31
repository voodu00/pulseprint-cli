use crate::config::PrinterConfig;
use rumqttc::{AsyncClient, EventLoop, MqttOptions, TlsConfiguration, Transport};
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::{DigitallySignedStruct, SignatureScheme};
use std::error::Error;
use std::time::Duration;

#[cfg(test)]
mod tests;

pub mod subscription;

// Custom certificate verifier that accepts any certificate (for local printers with self-signed certs)
#[derive(Debug)]
struct NoVerifyTls {}

impl ServerCertVerifier for NoVerifyTls {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA1,
            SignatureScheme::ECDSA_SHA1_Legacy,
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::ED25519,
            SignatureScheme::ED448,
        ]
    }
}

pub struct MqttClient {
    client: AsyncClient,
    eventloop: EventLoop,
}

impl MqttClient {
    pub async fn new(config: PrinterConfig) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut mqtt_options = MqttOptions::new("pulseprint-cli", &config.ip, config.port);

        // Set authentication
        mqtt_options.set_credentials("bblp", &config.access_code);

        // Configure TLS if enabled
        if config.use_tls {
            let tls_config = rustls::ClientConfig::builder()
                .dangerous()
                .with_custom_certificate_verifier(std::sync::Arc::new(NoVerifyTls {}))
                .with_no_client_auth();

            let tls_config = TlsConfiguration::Rustls(std::sync::Arc::new(tls_config));
            mqtt_options.set_transport(Transport::Tls(tls_config));
        }

        // Set connection parameters
        mqtt_options.set_keep_alive(Duration::from_secs(30));

        let (client, eventloop) = AsyncClient::new(mqtt_options, 10);

        Ok(MqttClient { client, eventloop })
    }

    pub fn into_parts(self) -> (AsyncClient, EventLoop) {
        (self.client, self.eventloop)
    }
}
