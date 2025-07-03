# CartoQuantum Protocol - Whitepaper

**Version:** 1.0  
**Author:** dfralan
**Date of Publication:** 07/01/2025

---

## 1. Overview

**CartoQuantum Protocol** defines a new method to encode, distribute, and validate digital objects in a fully self-contained, decentralized, and optionally ephemeral manner.

The protocol combines:

- Semantic and temporal addressing ("cartographic" coordinates).
- An algebra of operators that describe object state transformations.
- Compression and encoding techniques to embed all data into a single URL or QR code.

This system allows the creation of *stateful digital objects* that can be transmitted without requiring central servers, enabling intractable sharing, ephemeral publishing, and programmable content evolution.

---

## 2. Core Concepts

### 2.1 Envelope Structure

Each CartoQuantum Object is wrapped in an Envelope with explicit protocol metadata:

```json
{
  "envelope": {
    "protocol": "cartoquantum",
    "version": 1,
    "encoding": "deflate+base64url",
    "issued_at": 1729341900,
    "sig": "optional-signature"
  },
  "object": {
    "kind": "note",
    "coords": {
      "time": 1729342000,
      "region": "AR-BA",
      "context": "note",
      "author": "npub1xyz..."
    },
    "ops": ["⊕channel123", "⊗v2"],
    "expires": 1729345000,
    "payload": {
      "content": "Hola mundo cartocuántico"
    }
  }
}

```

Envelope fields:

protocol: Always cartoquantum.

version: Protocol version number.

encoding: Compression and encoding strategy.

issued_at: Unix timestamp when the envelope was created.

sig: Optional cryptographic signature.

Object fields:

kind: Type of object (note, payment, etc.).

coords: Semantic and temporal coordinates.

ops: Algebraic operators.

expires: Optional expiration timestamp.

payload: Arbitrary content.

This separation enables flexible validation, signatures, and compatibility across protocol versions.

---

### 2.2 Algebra of Operators

The **CartoQuantum Algebra** defines symbolic operators to manipulate object state:

| Operator | Meaning |
|---|---|
| ⊕ | Combination (merging with another object/channel) |
| ⊗ | Expansion (creating a version or derivative) |
| ⊖ | Invalidation (revoking or disabling previous state) |
| ⊘ | Projection (publishing into another context/namespace) |

Operators can be composed to express complex workflows without requiring external infrastructure.

---

### 2.3 Compression and Encoding

CartoQuantum Objects are:

1. **Serialized** as JSON.
2. **Compressed** using Deflate or Brotli.
3. **Base64url encoded**.
4. Embedded in the URL path or fragment.

This allows fully self-contained links such as:

https://cartoquantum.com/hash/eJxNj0FuAjEMRa-CvE4XVANTzY6uumaLEDKJRdNm7JA4oyI0h-EsXKzJbNqlv977X74D8URBIsFwh5hExUqAASwmlWtB1jKCgYlS9sIwrE01rDjPl0rFgJ7PmGnbVcjnXMidUCvWb9bda__W9bMBOX-R1Tbw7dlVjUWp8lYkudxy9SP9lwwkuix7sNu_vO8WmJV-9M_Gop-S6o0sfBulZGhbsRYejgYi3oKga-2Lyk39kICrsbCT1fKgLc8Hq7cC8zz_An1FWYk


No server storage is required to decode and interpret the object.

---

## 3. Example Envelope

```json
{
  "envelope": {
    "protocol": "cartoquantum",
    "version": 1,
    "encoding": "deflate+base64url",
    "issued_at": 1729341900
  },
  "object": {
    "kind": "note",
    "coords": {
      "time": 1729342000,
      "region": "AR-BA",
      "context": "note",
      "author": "npub1xyz..."
    },
    "ops": ["⊕channel123", "⊗v2"],
    "expires": 1729345000,
    "payload": {
      "content": "Hola mundo cartocuántico"
    }
  }
}

```

## 4. Lifecycle and Validation

Decoding:

Decompress and parse JSON.

Protocol Validation:

Verify protocol equals cartoquantum.

Check version compatibility.

Confirm encoding is supported.

Expiration Check:

If expires < current time, the object is invalid.

Algebra Application:

Interpret ops to determine object behavior.

Signature Verification:

Optionally validate sig against the author's public key.

## 5. Use Cases

Ephemeral notes shared without trace.

Payment links with embedded metadata and expiry.

Versioned publications.

Event broadcasting across contexts.

Fully client-side QR distribution.

## 6. License and Intellectual Property
The protocol description contained herein is published under the Creative Commons Attribution 4.0 International License (CC BY 4.0).

However, implementations of advanced validation logic, signature schemes, and operator resolution are reserved and may be released under separate licenses.

## 7. Contact and Attribution

**Author:** dfralan
**Repository:** https://github.com/dfralan/cartoquantum-protocol
**Date of Publication:** 07/01/2025

CartoQuantum Protocol - Ever Ephemeral. Never Ephemeral.
