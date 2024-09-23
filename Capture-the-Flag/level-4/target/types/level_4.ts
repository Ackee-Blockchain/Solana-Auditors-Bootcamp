/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/level_4.json`.
 */
export type Level4 = {
  "address": "D51vhx6jAbBtQVwo1fcYr7RMKQKAAnSUy6v7vRCHCZL3",
  "metadata": {
    "name": "level4",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "initVesting",
      "discriminator": [
        119,
        192,
        67,
        41,
        47,
        82,
        152,
        27
      ],
      "accounts": [
        {
          "name": "sender",
          "writable": true,
          "signer": true
        },
        {
          "name": "senderTokenAccount",
          "writable": true
        },
        {
          "name": "escrow",
          "writable": true,
          "signer": true
        },
        {
          "name": "escrowTokenAccount",
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
        }
      ],
      "args": [
        {
          "name": "recipient",
          "type": "pubkey"
        },
        {
          "name": "amount",
          "type": "u64"
        },
        {
          "name": "startAt",
          "type": "u64"
        },
        {
          "name": "endAt",
          "type": "u64"
        },
        {
          "name": "interval",
          "type": "u64"
        }
      ]
    },
    {
      "name": "revealSecret",
      "discriminator": [
        126,
        156,
        142,
        60,
        92,
        135,
        177,
        144
      ],
      "accounts": [
        {
          "name": "hacker",
          "signer": true
        },
        {
          "name": "hackerTokenAccount"
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
      "name": "withdrawUnlocked",
      "discriminator": [
        213,
        161,
        76,
        199,
        38,
        28,
        209,
        80
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
          "name": "tokenProgram"
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": []
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
      "name": "invalidAmount",
      "msg": "Invalid Amount!"
    },
    {
      "code": 6001,
      "name": "invalidTimeRange",
      "msg": "Invalid Time Range!"
    },
    {
      "code": 6002,
      "name": "invalidInterval",
      "msg": "Invalid Interval!"
    },
    {
      "code": 6003,
      "name": "overflow",
      "msg": "Overflow occurred!"
    },
    {
      "code": 6004,
      "name": "underflow",
      "msg": "Underflow occurred!"
    },
    {
      "code": 6005,
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
            "name": "recipient",
            "type": "pubkey"
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
            "name": "withdrawal",
            "type": "u64"
          },
          {
            "name": "startTime",
            "type": "u64"
          },
          {
            "name": "endTime",
            "type": "u64"
          },
          {
            "name": "interval",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
