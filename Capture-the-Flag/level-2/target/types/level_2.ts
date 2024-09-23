/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/level_2.json`.
 */
export type Level2 = {
  "address": "5pFB5g5WvBcL5fkaLY5Nz9cqEAxnkRPVgc3DCTKFY7uK",
  "metadata": {
    "name": "level2",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "battleMonster",
      "discriminator": [
        249,
        81,
        140,
        116,
        140,
        243,
        91,
        70
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "explorerAccount",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "healAlly",
      "discriminator": [
        52,
        27,
        215,
        189,
        71,
        155,
        112,
        241
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "injuredExplorer",
          "writable": true
        },
        {
          "name": "healer",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "initExplorer",
      "discriminator": [
        28,
        225,
        143,
        179,
        26,
        191,
        205,
        135
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "explorerAccount",
          "writable": true
        },
        {
          "name": "systemProgram"
        }
      ],
      "args": [
        {
          "name": "secret1",
          "type": "string"
        },
        {
          "name": "secret2",
          "type": "string"
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
          "name": "user1",
          "signer": true
        },
        {
          "name": "user2",
          "signer": true
        },
        {
          "name": "explorer1Account",
          "writable": true
        },
        {
          "name": "explorer2Account",
          "writable": true
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "explorer",
      "discriminator": [
        131,
        148,
        73,
        106,
        166,
        178,
        64,
        42
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "zombiesNotAllowed",
      "msg": "Explorear already died!"
    },
    {
      "code": 6001,
      "name": "notEnoughMana",
      "msg": "Not enough mana!"
    },
    {
      "code": 6002,
      "name": "incorrectSecrets",
      "msg": "You need to obtain secrets from the previous level first!"
    },
    {
      "code": 6003,
      "name": "sameExplorersNotAllowed",
      "msg": "Two identical explorers detected!"
    },
    {
      "code": 6004,
      "name": "notEnoughExperience",
      "msg": "Not enough experience points to reveal the Secret!"
    },
    {
      "code": 6005,
      "name": "notEnoughMonstersDefeated",
      "msg": "Not enough monsters defeated to reveal the Secret!"
    }
  ],
  "types": [
    {
      "name": "explorer",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mana",
            "type": "u8"
          },
          {
            "name": "health",
            "type": "u8"
          },
          {
            "name": "experience",
            "type": "u8"
          },
          {
            "name": "monstersDefeated",
            "type": "u8"
          }
        ]
      }
    }
  ]
};
