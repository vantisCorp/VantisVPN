/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  tutorialSidebar: [
    {
      type: 'category',
      label: 'Getting Started',
      items: [
        'intro',
        'installation',
        'quick-start',
        'configuration',
      ],
    },
    {
      type: 'category',
      label: 'Features',
      items: [
        {
          type: 'category',
          label: 'Security',
          items: [
            'security/overview',
            'security/post-quantum-cryptography',
            'security/zero-trust-architecture',
            'security/privacy-features',
          ],
        },
        {
          type: 'category',
          label: 'Networking',
          items: [
            'networking/wireguard',
            'networking/quic',
            'networking/protocols',
          ],
        },
        {
          type: 'category',
          label: 'Privacy',
          items: [
            'privacy/no-logs',
            'privacy/dns-over-https',
            'privacy/obfuscation',
          ],
        },
      ],
    },
    {
      type: 'category',
      label: 'Architecture',
      items: [
        'architecture/overview',
        'architecture/microservices',
        'architecture/security',
        'architecture/networking',
      ],
    },
    {
      type: 'category',
      label: 'API Reference',
      items: [
        'api/overview',
        'api/rest-api',
        'api/websocket-api',
        'api/webhooks',
      ],
    },
    {
      type: 'category',
      label: 'Development',
      items: [
        'development/setup',
        'development/testing',
        'development/contributing',
        'development/code-style',
      ],
    },
    {
      type: 'category',
      label: 'Deployment',
      items: [
        'deployment/production',
        'deployment/docker',
        'deployment/kubernetes',
        'deployment/cloud',
      ],
    },
    {
      type: 'category',
      label: 'Operations',
      items: [
        'operations/monitoring',
        'operations/logging',
        'operations/troubleshooting',
        'operations/performance',
      ],
    },
    {
      type: 'category',
      label: 'Compliance',
      items: [
        'compliance/privacy-by-design',
        'compliance/gdpr',
        'compliance/hipaa',
        'compliance/iso-27001',
      ],
    },
  ],
};

module.exports = sidebars;