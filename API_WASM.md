# WASM API for BRC-100 Web Components

JavaScript/TypeScript API for web applications.

## WalletWeb Class

```typescript
export class WalletWeb {
  constructor(config: WalletConfig);
  init(): Promise<void>;
  
  // Actions
  createAction(args: CreateActionArgs): Promise<CreateActionResult>;
  signAction(args: SignActionArgs): Promise<SignActionResult>;
  internalizeAction(args: InternalizeActionArgs): Promise<InternalizeActionResult>;
  
  // Queries
  listActions(args?: ListActionsArgs): Promise<ListActionsResult>;
  listOutputs(args?: ListOutputsArgs): Promise<ListOutputsResult>;
  listCertificates(args?: ListCertificatesArgs): Promise<ListCertificatesResult>;
  
  // Certificates
  acquireCertificate(args: AcquireCertificateArgs): Promise<AcquireCertificateResult>;
  proveCertificate(args: ProveCertificateArgs): Promise<ProveCertificateResult>;
  
  // Crypto
  createSignature(args: CreateSignatureArgs): Promise<CreateSignatureResult>;
  encrypt(args: WalletEncryptArgs): Promise<WalletEncryptResult>;
}
```

## Storage

Uses IndexedDB for browser-native storage.

## Usage

```typescript
const wallet = new WalletWeb({ chain: 'main', storageName: 'my-wallet' });
await wallet.init();

const result = await wallet.createAction({
  description: 'Send BSV',
  outputs: [{ satoshis: 1000, lockScript: '...' }]
});
```

See `pkg/wallet_web.d.ts` for complete TypeScript definitions.
