/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/level_5.json`.
 */
export type Level5 = {
  "address": "5p55Gzc2FwGGHGp6DmncJaw9gbhLRpzj6Uca8Rv1bRhd",
  "metadata": {
    "name": "level5",
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
          "name": "sender",
          "writable": true,
          "signer": true
        },
        {
          "name": "recipient"
        },
        {
          "name": "guardianSet",
          "writable": true
        },
        {
          "name": "escrow",
          "writable": true
        },
        {
          "name": "escrowTokenAccount",
          "writable": true,
          "signer": true
        },
        {
          "name": "escrowPdaAuthority"
        },
        {
          "name": "senderTokenAccount",
          "writable": true
        },
        {
          "name": "mint"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram"
        },
        {
          "name": "verificationProgram"
        }
      ],
      "args": [
        {
          "name": "guardianSetIndex",
          "type": "u32"
        },
        {
          "name": "signaturesNumber",
          "type": "u8"
        },
        {
          "name": "expirationTime",
          "type": "i64"
        },
        {
          "name": "signatures",
          "type": {
            "vec": "u64"
          }
        },
        {
          "name": "amountIn",
          "type": "u64"
        }
      ]
    },
    {
      "name": "obtainSecret",
      "discriminator": [
        123,
        215,
        246,
        168,
        114,
        9,
        183,
        93
      ],
      "accounts": [
        {
          "name": "hacker",
          "signer": true
        },
        {
          "name": "hackerAta"
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
    },
    {
      "name": "withdraw",
      "discriminator": [
        183,
        18,
        70,
        156,
        148,
        109,
        161,
        34
      ],
      "accounts": [
        {
          "name": "recipient",
          "writable": true,
          "signer": true
        },
        {
          "name": "recipientTokenAccount",
          "writable": true
        },
        {
          "name": "guardianSet",
          "docs": [
            "Will be checked in the CPI Program."
          ]
        },
        {
          "name": "escrow",
          "writable": true
        },
        {
          "name": "escrowTokenAccount",
          "writable": true
        },
        {
          "name": "escrowPdaAuthority"
        },
        {
          "name": "mint"
        },
        {
          "name": "verificationProgram"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram"
        },
        {
          "name": "associatedTokenProgram"
        }
      ],
      "args": [
        {
          "name": "passphrase",
          "type": {
            "vec": "u64"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "escrow",
      "discriminator": [
        31,
        213,
        123,
        187,
        186,
        22,
        218,
        155
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "withdrawWindowPassed",
      "msg": "Withdraw window passed!"
    },
    {
      "code": 6001,
      "name": "lengthsDoNotCorrespond",
      "msg": "Number of signatures needs to be the same as Vector Length!"
    },
    {
      "code": 6002,
      "name": "notEnoughFunds",
      "msg": "Not enough funds!"
    },
    {
      "code": 6003,
      "name": "pastNotAllowed",
      "msg": "Expiration Time cannot be in the past!"
    },
    {
      "code": 6004,
      "name": "incorrectSecrets",
      "msg": "You need to obtain secrets from the previous level first!"
    }
  ],
  "types": [
    {
      "name": "escrow",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "index",
            "type": "u32"
          },
          {
            "name": "sender",
            "type": "pubkey"
          },
          {
            "name": "recipient",
            "type": "pubkey"
          },
          {
            "name": "expirationTime",
            "type": "i64"
          },
          {
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u64"
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
