/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/level_3.json`.
 */
export type Level3 = {
  "address": "5zjbNpnsSkCNG6zHzK183ujm6dn6fWeHWeUnk1Rzrs1Y",
  "metadata": {
    "name": "level3",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
        {
          "name": "factionCreator",
          "writable": true,
          "signer": true
        },
        {
          "name": "faction",
          "writable": true
        },
        {
          "name": "mint",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram"
        },
        {
          "name": "tokenProgram"
        }
      ],
      "args": [
        {
          "name": "name",
          "type": "string"
        },
        {
          "name": "symbol",
          "type": "string"
        }
      ]
    },
    {
      "name": "obtainFactionToken",
      "discriminator": [
        85,
        134,
        20,
        48,
        83,
        187,
        45,
        162
      ],
      "accounts": [
        {
          "name": "factionAuthority",
          "signer": true
        },
        {
          "name": "faction"
        },
        {
          "name": "mint",
          "writable": true
        },
        {
          "name": "newMember",
          "writable": true,
          "signer": true
        },
        {
          "name": "newMemberTokenAccount",
          "writable": true
        },
        {
          "name": "systemProgram"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "associatedTokenProgram"
        }
      ],
      "args": []
    },
    {
      "name": "showFactionSecret",
      "discriminator": [
        211,
        66,
        73,
        2,
        244,
        189,
        57,
        188
      ],
      "accounts": [
        {
          "name": "factionMember",
          "signer": true
        },
        {
          "name": "faction"
        },
        {
          "name": "memberTokenAccount"
        },
        {
          "name": "mint"
        }
      ],
      "args": [
        {
          "name": "secret",
          "type": "string"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "faction",
      "discriminator": [
        131,
        20,
        223,
        22,
        227,
        204,
        231,
        35
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "incorrectSecret",
      "msg": "You need to obtain secret from the previous level first!"
    }
  ],
  "types": [
    {
      "name": "faction",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "membersCount",
            "type": "u64"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
