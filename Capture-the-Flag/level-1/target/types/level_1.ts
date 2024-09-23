/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/level_1.json`.
 */
export type Level1 = {
  "address": "7u6UPvrh9stwx6ApoYPyvn62ScvQtUc3fJgMQXKvbViK",
  "metadata": {
    "name": "level1",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "accessVault",
      "discriminator": [
        22,
        90,
        183,
        142,
        98,
        79,
        189,
        155
      ],
      "accounts": [
        {
          "name": "explorer",
          "writable": true,
          "signer": true
        },
        {
          "name": "ancientVault",
          "writable": true
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": [
        {
          "name": "pin",
          "type": "u8"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "ancientVault",
      "discriminator": [
        39,
        37,
        115,
        17,
        109,
        242,
        71,
        210
      ]
    }
  ],
  "types": [
    {
      "name": "ancientVault",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "secret2",
            "type": "string"
          }
        ]
      }
    }
  ]
};
