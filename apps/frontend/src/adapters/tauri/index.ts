// Tauri adapter - Desktop implementation
// This file re-exports all domain-specific modules

import type { RunEnv } from "../types";
import { RunEnvs } from "../types";

// Platform constants from core
export { invoke, isDesktop, isWeb, logger } from "./core";

/**
 * Runtime environment identifier - always "desktop" for Tauri builds
 */
export const RUN_ENV: RunEnv = RunEnvs.DESKTOP;

// Re-export types and constants from ../types
export { RunEnvs } from "../types";
export type {
  AddonFile,
  AddonInstallResult,
  AddonManifest,
  AddonUpdateCheckResult,
  AddonUpdateInfo,
  AddonValidationResult,
  AppInfo,
  BackendEnableSyncResult,
  BackendSyncBackgroundEngineResult,
  BackendSyncBootstrapOverwriteCheckResult,
  BackendSyncBootstrapResult,
  BackendSyncCycleResult,
  BackendSyncEngineStatusResult,
  BackendSyncReconcileReadyResult,
  BackendSyncSnapshotUploadResult,
  BackendSyncStateResult,
  EphemeralKeyPair,
  EventCallback,
  ExtractedAddon,
  FunctionPermission,
  ImportRunsRequest,
  InstalledAddon,
  Logger,
  MarketDataProviderSetting,
  Permission,
  PlatformCapabilities,
  PlatformInfo,
  ProviderCapabilities,
  RunEnv,
  UnlistenFn,
  UpdateCheckPayload,
  UpdateCheckResult,
  UpdateThreadRequest,
  UpdateToolResultRequest,
} from "../types";

// Re-export AI types from features/ai-assistant
export type {
  AiChatMessage,
  AiChatModelConfig,
  AiSendMessageRequest,
  AiStreamEvent,
  AiThread,
  AiToolCall,
  AiToolResult,
  AiUsageStats,
  ListThreadsRequest,
  ThreadPage,
} from "@/features/ai-assistant/types";

// ============================================================================
// Shared domain modules (identical logic for both platforms)
// ============================================================================

// Account Commands
export * from "../shared/accounts";

// Activity Commands
export * from "../shared/activities";
export { parseCsv } from "./activities";

// Portfolio Commands
export * from "../shared/portfolio";

// Market Data Commands
export * from "../shared/market-data";

// Custom Provider Commands
export * from "../shared/custom-provider";

// Goal Commands
export * from "../shared/goals";

// Taxonomy Commands
export * from "../shared/taxonomies";

// Alternative Assets Commands
export * from "../shared/alternative-assets";

// Contribution Limits Commands
export * from "../shared/contribution-limits";

// Exchange Rates Commands
export * from "../shared/exchange-rates";

// Secrets Commands
export * from "../shared/secrets";

// Connect Commands (Broker + Device Sync + Auth)
export * from "../shared/connect";

// AI Providers Commands
export * from "../shared/ai-providers";

// AI Thread Commands
export * from "../shared/ai-threads";

// Health Center Commands
export * from "../shared/health";

// ============================================================================
// Platform-specific modules (different implementations)
// ============================================================================

// Settings Commands (contains platform-specific backupDatabase, etc.)
export {
  backupDatabase,
  backupDatabaseToPath,
  checkForUpdates,
  getAppInfo,
  getPlatform,
  getSettings,
  installUpdate,
  isAutoUpdateCheckEnabled,
  restoreDatabase,
  updateSettings,
} from "./settings";

// Addon Commands (platform-specific)
export {
  checkAddonUpdate,
  checkAllAddonUpdates,
  clearAddonStaging,
  downloadAddonForReview,
  extractAddon,
  extractAddonZip,
  fetchAddonStoreListings,
  getAddonRatings,
  getEnabledAddons,
  getEnabledAddonsOnStartup,
  getInstalledAddons,
  installAddon,
  installAddonFile,
  installAddonZip,
  installFromStaging,
  listInstalledAddons,
  loadAddon,
  loadAddonForRuntime,
  submitAddonRating,
  toggleAddon,
  uninstallAddon,
  updateAddon,
} from "./addons";

// AI Streaming (Tauri Channel-based implementation)
export { streamAiChat } from "./ai-streaming";

// Event Listeners (Tauri listen() implementation)
export {
  listenBrokerSyncComplete,
  listenBrokerSyncError,
  listenBrokerSyncStart,
  listenDatabaseRestored,
  listenDeepLink,
  listenFileDrop,
  listenFileDropCancelled,
  listenFileDropHover,
  listenMarketSyncComplete,
  listenMarketSyncError,
  listenMarketSyncStart,
  listenNavigateToRoute,
  listenPortfolioUpdateComplete,
  listenPortfolioUpdateError,
  listenPortfolioUpdateStart,
} from "./events";

// File Dialogs (Tauri file dialogs)
export {
  openCsvFileDialog,
  openDatabaseFileDialog,
  openFileSaveDialog,
  openFolderDialog,
  openUrlInBrowser,
} from "./files";

// Crypto Commands (sync crypto operations)
export {
  syncComputeSas,
  syncComputeSharedSecret,
  syncDecrypt,
  syncDeriveDek,
  syncDeriveSessionKey,
  syncEncrypt,
  syncGenerateDeviceId,
  syncGenerateKeypair,
  syncGeneratePairingCode,
  syncGenerateRootKey,
  syncHashPairingCode,
  syncHmacSha256,
} from "./crypto";

// FIRE Planner (desktop-only feature)
export {
  calculateRetirementProjection,
  runRetirementDecisionSensitivityMap,
  runRetirementMonteCarlo,
  runRetirementScenarioAnalysis,
  runRetirementSorr,
  runRetirementStressTests,
} from "./fire-planner";
