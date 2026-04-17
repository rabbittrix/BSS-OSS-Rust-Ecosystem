/**
 * Minimal TypeScript client for bss-oss-pcf-nextgen REST API.
 * Requires global `fetch` (Node 18+, browsers, Deno).
 */

export type NetworkGeneration = "3G" | "4G" | "5G" | "6G";

export interface PolicyRequest {
  subscriber_id: string;
  imsi: string;
  network_generation: NetworkGeneration;
  apn: string;
  service_type: string;
  application_id?: string;
  location?: string;
}

export interface PolicyIntent {
  intent_id: string;
  description: string;
  target_latency_ms_p99?: number;
  min_downlink_mbps?: number;
  slice_hint?: string;
  application_id?: string;
}

export class PcfNextGenClient {
  constructor(
    private readonly baseUrl: string,
    private readonly accessToken?: string,
  ) {}

  private headers(): HeadersInit {
    const h: Record<string, string> = { Accept: "application/json" };
    if (this.accessToken) {
      h.Authorization = `Bearer ${this.accessToken}`;
    }
    return h;
  }

  async policyDecision(
    body: PolicyRequest & { tenant_id?: string },
  ): Promise<unknown> {
    const res = await fetch(`${this.baseUrl}/npcf-sba/v1/policy/decision`, {
      method: "POST",
      headers: { ...this.headers(), "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`${res.status} ${await res.text()}`);
    return res.json();
  }

  async policyIntent(body: {
    intent: PolicyIntent;
    session: PolicyRequest;
    tenant_id?: string;
  }): Promise<unknown> {
    const res = await fetch(`${this.baseUrl}/npcf-sba/v1/policy/intent`, {
      method: "POST",
      headers: { ...this.headers(), "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`${res.status} ${await res.text()}`);
    return res.json();
  }

  async monetizationQuote(body: {
    service_class: string;
    requested_downlink_mbps: number;
    expected_latency_ms_p99: number;
    duration_seconds: number;
    currency: string;
  }): Promise<unknown> {
    const res = await fetch(`${this.baseUrl}/nchf-ready/v1/quote`, {
      method: "POST",
      headers: { ...this.headers(), "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    if (!res.ok) throw new Error(`${res.status} ${await res.text()}`);
    return res.json();
  }
}
