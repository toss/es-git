[es-git](../globals.md) / Credential

# Type Alias: Credential

> **Credential**: \{ `type`: `"Default"`; \} \| \{ `type`: `"SSHKeyFromAgent"`; `username`: `string`; \} \| \{ `type`: `"SSHKeyFromPath"`; `username`: `string`; `publicKeyPath`: `string`; `privateKeyPath`: `string`; `passphrase`: `string`; \} \| \{ `type`: `"SSHKey"`; `username`: `string`; `publicKey`: `string`; `privateKey`: `string`; `passphrase`: `string`; \} \| \{ `type`: `"Plain"`; `username`: `string`; `password`: `string`; \}
