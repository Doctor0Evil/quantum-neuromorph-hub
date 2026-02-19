class TransparencyPortal {
  constructor(config) {
    this.config = config;
    this.state = {
      metrics: null,
      errors: [],
      lastUpdate: null
    };
  }

  async fetchJson(path) {
    const resp = await fetch(`${this.config.baseUrl}${path}`, {
      headers: { "Accept": "application/json" }
    });
    if (!resp.ok) {
      throw new Error(`HTTP ${resp.status}: ${await resp.text()}`);
    }
    return resp.json();
  }

  async refreshMetrics() {
    try {
      const metrics = await this.fetchJson(
        `/neighborhoods/${encodeURIComponent(this.config.neighborhoodId)}/metrics`
      );
      this.state.metrics = metrics;
      this.state.lastUpdate = new Date();
      this.render();
    } catch (e) {
      console.error("Metrics fetch failed:", e);
      this.state.errors.push({ ts: new Date(), message: e.message });
      this.renderErrors();
    }
  }

  render() {
    const root = document.getElementById("portal-root");
    if (!root || !this.state.metrics) return;

    const m = this.state.metrics;
    root.innerHTML = `
      <h2>Neighborhood: ${m.name}</h2>
      <p>Last update: ${this.state.lastUpdate.toISOString()}</p>
      <h3>Environmental Metrics</h3>
      <ul>
        <li>PM2.5: ${m.air.pm25} µg/m³</li>
        <li>NO₂: ${m.air.no2} ppb</li>
        <li>Water Lead: ${m.water.lead_ppb} ppb</li>
        <li>Heat Index: ${m.climate.heat_index_c} °C</li>
      </ul>
      <h3>Karma & Safety</h3>
      <ul>
        <li>Karma score: ${m.karma.score}</li>
        <li>Recent veto events: ${m.karma.veto_events}</li>
        <li>BCI safety incidents (last 24h): ${m.safety.bci_incidents}</li>
      </ul>
    `;
  }

  renderErrors() {
    const el = document.getElementById("portal-errors");
    if (!el) return;
    el.innerHTML = this.state.errors
      .slice(-5)
      .map(e => `<li>[${e.ts.toISOString()}] ${e.message}</li>`)
      .join("");
  }

  start() {
    this.refreshMetrics();
    setInterval(() => this.refreshMetrics(), this.config.refreshMs || 60000);
  }
}

// Usage:
// const portal = new TransparencyPortal({ baseUrl: "https://phx-viva-la.city/api", neighborhoodId: "phx-west-001", refreshMs: 30000 });
// portal.start();
