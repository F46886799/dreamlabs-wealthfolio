import React, { useEffect, useState } from "react";
import { type AddonContext, type ExchangeRate } from "@wealthfolio/addon-sdk";
import { Calculator } from "lucide-react";
import { QueryClientProvider, type QueryClient } from "@tanstack/react-query";
import { Page, PageContent, PageHeader } from "@wealthfolio/ui";

const FxCalculatorPage = ({ ctx }: { ctx: AddonContext }) => {
  const [apiKey, setApiKey] = useState("");
  const [isKeySaved, setIsKeySaved] = useState(false);
  const [currentRates, setCurrentRates] = useState<ExchangeRate[]>([]);
  const [fromCurrency, setFromCurrency] = useState("USD");
  const [toCurrency, setToCurrency] = useState("EUR");
  const [amount, setAmount] = useState<number>(100);
  const [liveRate, setLiveRate] = useState<number | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Load API Key and current rates on mount
  useEffect(() => {
    const init = async () => {
      try {
        const savedKey = await ctx.api.secrets.get("FX_API_KEY");
        if (savedKey) {
          setApiKey(savedKey);
          setIsKeySaved(true);
        }

        const rates = await ctx.api.exchangeRates.getAll();
        setCurrentRates(rates);
      } catch (err) {
        ctx.api.logger.error("Error initializing FX Calculator: " + err);
      }
    };
    init();
  }, [ctx]);

  const handleSaveKey = async () => {
    try {
      await ctx.api.secrets.set("FX_API_KEY", apiKey);
      setIsKeySaved(true);
      if (ctx.api.toast) ctx.api.toast.success("API Key saved securely.");
    } catch (err) {
      if (ctx.api.toast) ctx.api.toast.error("Failed to save API key.");
    }
  };

  const clearKey = async () => {
    try {
      await ctx.api.secrets.delete("FX_API_KEY");
      setApiKey("");
      setIsKeySaved(false);
      if (ctx.api.toast) ctx.api.toast.info("API Key removed.");
    } catch (err) {
      if (ctx.api.toast) ctx.api.toast.error("Failed to remove API key.");
    }
  };

  const handleCalculate = async () => {
    if (!apiKey) {
      setError("Please save an API key first.");
      return;
    }

    setIsLoading(true);
    setError(null);
    setLiveRate(null);

    try {
      // ExchangeRate-API (v6) format
      const response = await fetch(
        `https://v6.exchangerate-api.com/v6/${apiKey}/pair/${fromCurrency}/${toCurrency}`
      );

      const data = await response.json();

      if (data.result === "success") {
        setLiveRate(data.conversion_rate);
        if (ctx.api.toast) ctx.api.toast.success(`Fetched live rate: ${data.conversion_rate}`);
      } else {
        setError(data["error-type"] || "Failed to fetch rate");
      }
    } catch (err: any) {
      setError("Network or API error: " + err.message);
    } finally {
      setIsLoading(false);
    }
  };

  const updateSystemRate = async () => {
    if (liveRate === null) return;

    try {
      // Find if we already have this rate to update it
      const existing = currentRates.find(
        (r) => r.fromCurrency === fromCurrency && r.toCurrency === toCurrency
      );

      if (existing) {
        await ctx.api.exchangeRates.update({
          ...existing,
          rate: liveRate,
          timestamp: new Date().toISOString(),
          source: "FX Addon"
        });
      } else {
        await ctx.api.exchangeRates.add({
          fromCurrency,
          toCurrency,
          rate: liveRate,
          timestamp: new Date().toISOString(),
          source: "FX Addon"
        });
      }

      // Refresh local rates
      const rates = await ctx.api.exchangeRates.getAll();
      setCurrentRates(rates);

      if (ctx.api.toast) ctx.api.toast.success(`Successfully updated system rate for ${fromCurrency}/${toCurrency}`);
    } catch (err: any) {
      if (ctx.api.toast) ctx.api.toast.error(`Failed to update system rate: ${err.message}`);
    }
  };

  return (
    <Page>
      <PageHeader>
        <div className="flex flex-col gap-2">
          <div className="flex items-center gap-2">
            <h1 className="text-lg font-semibold sm:text-xl">FX Calculator</h1>
          </div>
          <p className="text-muted-foreground text-sm sm:text-base">Calculate real-time exchange rates and update your system settings.</p>
        </div>
      </PageHeader>
      <PageContent>
        <div className="p-6 max-w-2xl mx-auto space-y-6">

          {/* Settings Section */}
          <div className="bg-white dark:bg-[#1C1F26] p-4 rounded-lg border border-gray-200 dark:border-gray-800 shadow-sm">
            <h2 className="text-lg font-semibold mb-2">API Settings</h2>
            <p className="text-sm text-gray-500 mb-4">
              Enter your ExchangeRate-API.com key. It will be stored securely using the host's keyring.
            </p>
            <div className="flex gap-2">
              <input
                type="password"
                className="flex-1 px-3 py-2 bg-gray-50 dark:bg-[#111216] border border-gray-200 dark:border-gray-800 rounded-md outline-none focus:ring-1 focus:ring-blue-500"
                placeholder="Enter API Key"
                value={apiKey}
                onChange={(e) => setApiKey(e.target.value)}
                disabled={isKeySaved}
              />
              {!isKeySaved ? (
                <button
                  onClick={handleSaveKey}
                  className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 font-medium"
                >
                  Save Key
                </button>
              ) : (
                <button
                  onClick={clearKey}
                  className="px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700 font-medium"
                >
                  Clear Key
                </button>
              )}
            </div>
          </div>

          {/* Calculator Section */}
          <div className="bg-white dark:bg-[#1C1F26] p-4 rounded-lg border border-gray-200 dark:border-gray-800 shadow-sm space-y-4">
            <h2 className="text-lg font-semibold">Calculator</h2>

            <div className="grid grid-cols-2 gap-4 pb-4">
              <div>
                <label className="block text-sm font-medium mb-1">From Currency</label>
                <input
                  type="text"
                  className="w-full px-3 py-2 bg-gray-50 dark:bg-[#111216] border border-gray-200 dark:border-gray-800 rounded-md outline-none text-transform: uppercase"
                  value={fromCurrency}
                  onChange={(e) => setFromCurrency(e.target.value.toUpperCase())}
                  placeholder="e.g. USD"
                  maxLength={3}
                />
              </div>
              <div>
                <label className="block text-sm font-medium mb-1">To Currency</label>
                <input
                  type="text"
                  className="w-full px-3 py-2 bg-gray-50 dark:bg-[#111216] border border-gray-200 dark:border-gray-800 rounded-md outline-none text-transform: uppercase"
                  value={toCurrency}
                  onChange={(e) => setToCurrency(e.target.value.toUpperCase())}
                  placeholder="e.g. EUR"
                  maxLength={3}
                />
              </div>
              <div className="col-span-2">
                <label className="block text-sm font-medium mb-1">Amount</label>
                <input
                  type="number"
                  className="w-full px-3 py-2 bg-gray-50 dark:bg-[#111216] border border-gray-200 dark:border-gray-800 rounded-md outline-none"
                  value={amount}
                  onChange={(e) => setAmount(Number(e.target.value))}
                  min={0}
                />
              </div>
            </div>

            {error && <div className="text-red-500 text-sm py-2">{error}</div>}

            <button
              onClick={handleCalculate}
              disabled={isLoading || !isKeySaved}
              className="w-full py-3 bg-blue-600 text-white font-medium rounded-md hover:bg-blue-700 disabled:bg-gray-400 dark:disabled:bg-gray-700 flex justify-center items-center"
            >
              {isLoading ? "Fetching..." : "Fetch Live Rate & Calculate"}
            </button>

            {liveRate !== null && (
              <div className="mt-6 p-4 bg-gray-50 dark:bg-[#111216] rounded-md border border-gray-200 dark:border-gray-800">
                <div className="text-sm text-gray-500">Live Rate</div>
                <div className="text-xl font-bold mb-4">1 {fromCurrency} = {liveRate.toFixed(4)} {toCurrency}</div>

                <div className="text-sm text-gray-500">Converted Amount</div>
                <div className="text-3xl font-bold text-green-600 dark:text-green-400">
                  {(amount * liveRate).toLocaleString(undefined, { maximumFractionDigits: 2 })} {toCurrency}
                </div>

                <div className="pt-6">
                  <button
                    onClick={updateSystemRate}
                    className="w-full py-2 bg-green-600 text-white rounded-md hover:bg-green-700 text-sm font-medium border-none cursor-pointer"
                  >
                    Update wealthfolio system rate to {liveRate.toFixed(4)}
                  </button>
                  <p className="text-xs text-gray-500 mt-2 text-center">
                    This will save the new rate to the core system database, overriding the previous manual/market rate.
                  </p>
                </div>
              </div>
            )}
          </div>
        </div>
      </PageContent>
    </Page>
  );
};

// ----------------------------------------------------------------------------------
// Entry Point
// ----------------------------------------------------------------------------------
export default function enable(ctx: AddonContext) {
  ctx.api.logger.info("FX Calculator addon is being enabled");

  const addedItems: Array<{ remove: () => void }> = [];

  try {
    // 1. Add Sidebar item
    const sidebarItem = ctx.sidebar.addItem({
      id: "fx-calculator",
      label: "FX Calculator",
      icon: <Calculator className="h-5 w-5" />,
      route: "/addon/fx-calculator",
      order: 150,
    });
    addedItems.push(sidebarItem);

    // Create wrapper component with QueryClientProvider using shared client
    const FxCalculatorWrapper = () => {
      const sharedQueryClient = ctx.api.query.getClient() as QueryClient;
      return (
        <QueryClientProvider client={sharedQueryClient}>
          <FxCalculatorPage ctx={ctx} />
        </QueryClientProvider>
      );
    };

    // 2. Register route
    ctx.router.add({
      path: "/addon/fx-calculator",
      component: React.lazy(() => Promise.resolve({
        default: FxCalculatorWrapper
      })),
    });

    ctx.api.logger.info("FX Calculator addon enabled successfully");
  } catch (err: any) {
    if (ctx.api.toast) {
      ctx.api.toast.error("Failed to initialize FX Calculator: " + err.message);
    }
  }

  // 3. Cleanup
  ctx.onDisable(() => {
    addedItems.forEach((item) => item.remove());
    ctx.api.logger.info("FX Calculator addon disabled");
  });
}