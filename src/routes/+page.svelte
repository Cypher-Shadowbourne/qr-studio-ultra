<script lang="ts">
  import { onMount, tick } from "svelte";
  import { fade, fly, scale } from "svelte/transition";
  import { backOut, cubicOut } from "svelte/easing";
  import { invoke } from "@tauri-apps/api/core";
  import { settingsStore } from "$lib/settingsStore.svelte";
  import { generateAiMagicDesign, getProviderLabel } from "$lib/aiService";
  import Settings from "./Settings.svelte";
  import { Format, scan, cancel, requestPermissions } from "@tauri-apps/plugin-barcode-scanner";
  import { save } from "@tauri-apps/plugin-dialog";

  type SavedWallet = {
    id: string;
    name: string;
    network: string;
    address: string;
    amount: string;
    label: string;
    message: string;
  };

  type StudioTemplate = {
    id: string;
    name: string;
    createdAt: string;
    kind?: "single" | "batch";
    settings: Record<string, any>;
  };

  type HistoryEntry = {
    id: string;
    label: string;
    dataType: string;
    payload: string;
    createdAt: string;
    score: number;
  };

  type BatchResult = {
    id: string;
    label: string;
    payload: string;
    image: string;
    score: number;
  };

  // Data Types & Variables
  let dataType = "URL";
  let qrData = "your text here";
  let wifiSsid = "";
  let wifiPass = "";
  
  let petName = "";
  let microchipNum = "";
  let ownerName = "";
  let ownerPhone = "";
  let ownerAddr = "";

  let vCardFirst = "";
  let vCardLast = "";
  let vCardOrg = "";
  let vCardPhone = "";
  let vCardEmail = "";

  let emailTo = "";
  let emailSub = "";
  let emailBody = "";

  let smsPhone = "";
  let smsMsg = "";

  let phoneNum = "";
  let geoLat = "";
  let geoLng = "";

  let waPhone = "";
  let waMsg = "";

  let cryptoAddr = "";
  let cryptoType = "bitcoin";
  let cryptoAmount = "";
  let cryptoLabel = "";
  let cryptoMessage = "";
  let walletName = "";

  let eventTitle = "";
  let eventStart = "";
  let eventEnd = "";
  let eventLoc = "";

  let socialUser = "";
  let socialPlatform = "Facebook";

  let zoomMeetingId = "";
  let zoomPass = "";

  let linkedinUser = "";

  let ytHandle = "";

  let tiktokUser = "";

  // ✨ AI Magic State
  let aiMagicPrompt = "";
  let aiMagicLoading = false;
  let aiMagicForceCenterImage = true;
  let aiMagicLastCenterImagePrompt = "";
  let aiMagicActiveProvider = settingsStore.preferredAiProvider;
  let showAiMagic = false;
  let currentView = "studio"; // "studio" or "settings"
  const aiExamples = [
    "🌿 Emerald forest oracle, glowing runes, gold fireflies, soft teal mist, sacred botanical sigils",
    "🌙 Moonlit witch garden, amethyst petals, silver fog, luminous crystal altar, enchanted night bloom",
    "🦊 Cyberpunk spirit fox, holographic neon fur, violet aura, electric rain, chrome shrine reflections",
    "💎 Luxury black marble temple, liquid gold veins, royal purple glow, celestial halo, premium mystic depth",
    "🐉 Cosmic nebula dragon, teal stardust, ultraviolet wings, radiant eclipse ring, ancient astral symbols",
    "🔥 Volcanic phoenix sigil, obsidian shards, molten amber glow, smoke-lit feathers, dramatic inferno aura"
  ];

  async function handleAiMagicGenerate(moreFlair = false) {
    if (!aiMagicPrompt.trim()) return;
    
    let promptToSend = aiMagicPrompt;
    if (moreFlair) {
      promptToSend += " — more intense, more artistic, more over-the-top";
    }
    if (aiMagicForceCenterImage) {
      promptToSend += " — FORCE a detailed, vivid, artistic center_image_prompt for a beautiful center image/logo. Never leave center_image_prompt null, empty, generic, or vague.";
    }

    aiMagicLoading = true;
    try {
      const res = await generateAiMagicDesign(
        promptToSend,
        settingsStore.preferredAiProvider,
        settingsStore.aiApiKeys
      );
      aiMagicActiveProvider = res.provider_used ?? settingsStore.preferredAiProvider;
      
      // Apply Colors
      fillType = "Linear";
      color1 = res.gradient_start;
      color2 = res.gradient_mid;
      color3 = res.gradient_end;
      useFourthStop = false;
      
      ringColor = res.ring_color;
      const selectedRingStyle = chooseAiOverlayRingStyle(promptToSend, res.overall_mood);
      enableFrame = true;
      ringStyle = selectedRingStyle;
      ringColorMode = selectedRingStyle === "solid" ? "solid" : "gradient";
      ringGradientMode = "match-main";
      ringUseFourthStop = useFourthStop;
      transparentFrameBg = false;
      matchTextStyle = true;
      matchTextTopStyle = true;
      aiMagicLastCenterImagePrompt = res.center_image_prompt || "";
      if (aiMagicForceCenterImage && aiMagicLastCenterImagePrompt.trim()) {
        logoBase64 = createAiCenterImage(aiMagicLastCenterImagePrompt, res.overall_mood, color1, color2, color3);
        logoName = "AI Magic Center Image";
        logoTrimmedWidth = 512;
        logoTrimmedHeight = 512;
        logoSizePercent = 24;
        logoOpacityPercent = 100;
        centerOverlayMode = "custom";
        centerOverlayStyle = selectedRingStyle === "solid" ? "rounded" : selectedRingStyle;
        centerOverlayColor = ringColor;
        centerOverlayColor2 = color2;
        centerOverlayColor3 = color3;
        centerOverlayColor4 = color1;
        centerOverlayColorMode = centerOverlayStyle === "solid" ? "solid" : "gradient";
        centerOverlayGradientMode = "match-main";
      }

      // Apply Style mapping based on mood
      const mood = res.overall_mood.toLowerCase();
      if (mood.includes("cyberpunk") || mood.includes("neon") || mood.includes("futuristic") || mood.includes("vibrant")) {
        mainShape = "circle";
        bgShape = "rounded";
        eyeShape = "circle";
      } else if (mood.includes("luxurious") || mood.includes("elegant") || mood.includes("mystical")) {
        mainShape = "rounded";
        bgShape = "square";
        eyeShape = "rounded";
      } else if (mood.includes("playful") || mood.includes("chaotic")) {
        mainShape = "rounded";
        bgShape = "rounded";
        eyeShape = "circle";
      } else if (mood.includes("industrial") || mood.includes("dark")) {
        mainShape = "square";
        bgShape = "square";
        eyeShape = "square";
      }

      // Apply Text
      if (res.curved_text_top) {
        enableFrame = true;
        frameTextTop = res.curved_text_top;
        frameTextTopMode = "curved";
        frameTextTopColor = color1;
      }
      if (res.curved_text_bottom) {
        enableFrame = true;
        frameText = res.curved_text_bottom;
        frameTextMode = "curved";
        frameTextColor = color1;
      }

      showAiMagic = false;
      showSaveToastMessage(`Applied ${res.overall_mood} design with ${toDisplayCase(selectedRingStyle)} overlay`, "success");
      await tick();
      runGeneration();
    } catch (err: any) {
      showSaveToastMessage(err.toString(), "error");
    } finally {
      aiMagicLoading = false;
    }
  }

  // Main & Background
  let bgShape = "square";
  let mainShape = "square";
  let fillType = "Solid";
  let color1 = "#000000";
  let color2 = "#1a73e8";
  let color3 = "#ff416c";
  let color4 = "#ffd166";
  let useFourthStop = true;
  let bgColor = "#FFFFFF";

  // Eyes
  let eyeShape = "square";
  let eyeOut = "#000000";
  let eyeIn =  "#000000";

  // PRO Settings & Logo
  let enableFrame = false;
  let frameText = "Scan Me";
  let frameTextTop = "";
  let frameTextMode = "flat";
  let frameTextRadius = 350;
  let frameTextSpacing = 1;
  let frameTextSize = 44;
  let frameTextColor = "#000000";
  let matchTextStyle = false;
  let transparentTextBg = false;

  let frameTextTopMode = "flat";
  let frameTextTopRadius = 350;
  let frameTextTopSpacing = 1;
  let frameTextTopSize = 44;
  let frameTextTopColor = "#000000";
  let matchTextTopStyle = false;
  let transparentTextTopBg = false;

  let ringStyle = "solid";
  let ringColor = "#000000";
  let ringColor2 = "#8b5e3c";
  let ringColor3 = "#e6a756";
  let ringColor4 = "#ffd166";
  let ringUseFourthStop = true;
  let ringGradientMode = "match-main";
  let ringColorMode = "solid";
  let transparentFrameBg = false;
  let centerOverlayMode = "none";
  let centerOverlayStyle = "solid";
  let centerOverlayColor = "#000000";
  let centerOverlayColor2 = "#8b5e3c";
  let centerOverlayColor3 = "#e6a756";
  let centerOverlayColor4 = "#ffd166";
  let centerOverlayUseFourthStop = true;
  let centerOverlayGradientMode = "match-outer";
  let centerOverlayColorMode = "solid";
  const baseQrCanvasSize = 600;
  let logoSizePercent = 22;
  let logoOpacityPercent = 100;

  let generationTimer: ReturnType<typeof setTimeout> | null = null;
  const sanitizeShape = (shape: string, fallback = "square") =>
    shape === "circle" || shape === "rounded" || shape === "square" ? shape : fallback;
  const sanitizeFillType = (value: string) =>
    value === "Linear" || value === "Linear Gradient" ? "Linear" : "Solid";
  const sanitizeColorMode = (value: string) =>
    value?.toLowerCase() === "gradient" ? "gradient" : "solid";

  function hashText(value: string) {
    let hash = 0;
    for (let i = 0; i < value.length; i += 1) {
      hash = ((hash << 5) - hash + value.charCodeAt(i)) | 0;
    }
    return Math.abs(hash);
  }

  function chooseAiOverlayRingStyle(prompt: string, mood: string) {
    const text = `${prompt} ${mood}`.toLowerCase();
    if (/(neon|cyber|electric|holographic|glow|laser)/.test(text)) return "neon";
    if (/(luxury|luxurious|premium|gold|marble|elegant|royal)/.test(text)) return "double";
    if (/(space|cosmic|nebula|aurora|galaxy|dream)/.test(text)) return "gradient";
    if (/(candy|playful|bubble|confetti|dots|pop)/.test(text)) return "dotted";
    if (/(fire|lava|volcanic|industrial|warning|signal)/.test(text)) return "dashed";
    if (/(crystal|gem|diamond|sharp|prism)/.test(text)) return "diamond";
    if (/(forest|organic|mystical|leaf|soft|calm)/.test(text)) return "rounded";
    if (/(minimal|plain|clean|simple)/.test(text)) return "solid";

    const styles = ["solid", "double", "dotted", "dashed", "rounded", "diamond", "gradient", "neon"];
    return styles[hashText(text) % styles.length];
  }

  function hasAnyAiProviderKey() {
    return Object.values(settingsStore.aiApiKeys).some((key) => key.trim());
  }

  function debouncedRunGeneration() {
    if (generationTimer) clearTimeout(generationTimer);
    generationTimer = setTimeout(() => {
      runGeneration();
    }, 100);
  }

  let initialLoadDone = false;
  onMount(() => {
    initialLoadDone = true;
    
    const handleKeydown = (e: KeyboardEvent) => {
      // CMD/CTRL + S to Save PNG
      if ((e.metaKey || e.ctrlKey) && e.key === 's') {
        e.preventDefault();
        if (qrImagePng) quickSave("png");
      }
      // CMD/CTRL + G to Generate
      if ((e.metaKey || e.ctrlKey) && e.key === 'g') {
        e.preventDefault();
        manualGenerationRequested = true;
        runGeneration();
      }
      // ESC to close panels or modals
      if (e.key === 'Escape') {
        showHistoryPanel = false;
        showScanDecryptModal = false;
        showCropModal = false;
      }
    };

    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });

  $: if (initialLoadDone && typeof runGeneration === 'function') {
      // Trigger update on any of these variables
      const _trigger = [
        dataType, qrData, wifiSsid, wifiPass,
        petName, microchipNum, ownerName, ownerPhone, ownerAddr,
        vCardFirst, vCardLast, vCardOrg, vCardPhone, vCardEmail,
        emailTo, emailSub, emailBody, smsPhone, smsMsg, phoneNum, geoLat, geoLng,
        waPhone, waMsg, cryptoAddr, cryptoType, cryptoAmount, cryptoLabel, cryptoMessage,
        eventTitle, eventStart, eventEnd, eventLoc, socialUser, socialPlatform,
        zoomMeetingId, zoomPass, linkedinUser, ytHandle, tiktokUser,
        enableFrame, 
        frameText, frameTextMode, frameTextSize, frameTextColor, matchTextStyle, transparentTextBg,
        frameTextRadius, frameTextSpacing,
        frameTextTop, frameTextTopMode, frameTextTopSize, frameTextTopColor, matchTextTopStyle, transparentTextTopBg,
        frameTextTopRadius, frameTextTopSpacing,
        bgShape, mainShape, fillType, color1, color2, color3, color4, useFourthStop, bgColor,
        eyeShape, eyeOut, eyeIn, logoBase64, logoSizePercent, logoOpacityPercent,
        ringStyle, ringColor, ringColor2, ringColor3, ringColor4, ringUseFourthStop,
        ringColorMode, ringGradientMode, centerOverlayMode, centerOverlayStyle,
        centerOverlayColor, centerOverlayColor2, centerOverlayColor3, centerOverlayColor4,
        centerOverlayUseFourthStop, centerOverlayGradientMode, centerOverlayColorMode,
        transparentFrameBg
      ];
      if (buildFinalQrData().trim()) debouncedRunGeneration();
  }
  $: bgShape = sanitizeShape(bgShape, "square");
  $: mainShape = sanitizeShape(mainShape, "square");
  $: eyeShape = sanitizeShape(eyeShape, "square");
  $: fillType = sanitizeFillType(fillType);
  $: ringColorMode = sanitizeColorMode(ringColorMode);
  $: centerOverlayColorMode = sanitizeColorMode(centerOverlayColorMode);
  let logoName = "";
  let logoBase64 = "";
  let logoTrimmedWidth = 200;
  let logoTrimmedHeight = 200;
  let fileInput: HTMLInputElement;
  let fallbackColorInput: HTMLInputElement;

  // Render variables
  let qrImagePng = ""; 
  let qrImageJpg = ""; 
  let saveFormat = "png"; 
  let loading = false;
  let mobileSaveMessage = "";
  let showMobileSaveActions = false;
  let showSaveToast = false;
  let saveToastTone: "success" | "error" | "info" = "success";
  let saveToastTimer: ReturnType<typeof setTimeout> | null = null;
  let recentSaves: { label: string; timestamp: string }[] = [];
  let savedTemplates: StudioTemplate[] = [];
  let templateName = "";
  let generationHistory: HistoryEntry[] = [];
  let showHistoryPanel = false;
  let batchInput = "";
  let batchResults: BatchResult[] = [];
  let batchBusy = false;
  let batchProgress = 0;
  let batchTotal = 0;
  let manualGenerationRequested = false;
  let currentScannability = { score: 0, label: "Waiting", className: "empty", notes: ["Add content to score scannability."] };
  let scannabilityInput = "";
  let generatedPayload = "";
  let generatedLabel = "QR Code";
  let printTitle = "";
  let generatedAt = "";
  let encryptPlaintext = "";
  let encryptPassphrase = "";
  let showEncryptPassphrase = false;
  let encryptionAlgorithm = "chacha20-poly1305";
  let encryptedPayload = "";
  let encryptedForensicDate = "";
  let decryptPayload = "";
  let decryptPassphrase = "";
  let showDecryptPassphrase = false;
  let decryptedPlaintext = "";
  let decryptedForensicDate = "";
  let cryptoBusy = false;
  let scanDecryptSuccessTimer: ReturnType<typeof setTimeout> | null = null;
  let encryptStrength = { label: "Not set", className: "empty", percent: 0 };
  $: encryptStrength = getPassphraseStrength(encryptPassphrase);
  $: scannabilityInput = [
    dataType,
    qrData,
    wifiSsid,
    wifiPass,
    petName,
    microchipNum,
    ownerName,
    ownerPhone,
    ownerAddr,
    vCardFirst,
    vCardLast,
    vCardOrg,
    vCardPhone,
    vCardEmail,
    emailTo,
    emailSub,
    emailBody,
    smsPhone,
    smsMsg,
    phoneNum,
    geoLat,
    geoLng,
    waPhone,
    waMsg,
    cryptoAddr,
    cryptoType,
    cryptoAmount,
    cryptoLabel,
    cryptoMessage,
    walletName,
    eventTitle,
    eventStart,
    eventEnd,
    eventLoc,
    socialPlatform,
    linkedinUser,
    ytHandle,
    tiktokUser,
    zoomMeetingId,
    zoomPass,
    color1,
    color2,
    color3,
    color4,
    useFourthStop,
    bgColor,
    fillType,
    mainShape,
    logoBase64,
    logoSizePercent,
    enableFrame,
    transparentFrameBg
  ].join("|");
  $: currentScannability = getScannabilityScore(buildFinalQrData(), scannabilityInput);
  
  // Scanner & Modal State
  let isScanning = false; 
  let scannedResult = ""; 
  let scannedFormat = "";
  let scannedKind = "";
  let scannedResultCanOpen = false;
  let showScanDecryptModal = false;
  let scanEncryptedPayload = "";
  let scanDecryptPassphrase = "";
  let showScanDecryptPassphrase = false;
  let scanDecryptedPlaintext = "";
  let scanDecryptedForensicDate = "";
  let showScanDecryptSuccessBanner = false;
  let scanDecryptError = "";
  let showDogTagWarning = false; 
  let savedWallets: SavedWallet[] = [];
  let activeColorTarget = "color1";
  let recentColors: string[] = [];
  const templateStorageKey = "qr-studio-ultra.templates";
  const historyStorageKey = "qr-studio-ultra.history";
  $: currentScannability = getScannabilityScore(buildFinalQrData());

  // --- NEW: Interactive Crop State ---
  let showCropModal = false;
  let cropRawSrc = "";
  let cropImgEl: HTMLImageElement;
  let cropImgNaturalW = 0;
  let cropImgNaturalH = 0;
  let cropSize = 0;
  let cropX = 0;
  let cropY = 0;
  let cropZoom = 1.0;
  let isDragging = false;
  let dragStartX = 0;
  let dragStartY = 0;
  let cropStartX = 0;
  let cropStartY = 0;

  const solidColors = [
    "#000000", "#FFFFFF", "#555555", "#FF0000", "#00FF00", "#0000FF", "#FFFF00", "#00FFFF",
    "#FF00FF", "#800000", "#808000", "#008000", "#800080", "#000080", "#FFA500"
  ];

  const gradientPresets = [
    { name: "Sunset", c1: "#4b134f", c2: "#ff5f6d", c3: "#ffe082" },
    { name: "Ocean", c1: "#0f2027", c2: "#2c5364", c3: "#6dd5ed" },
    { name: "Forest", c1: "#0b3d2e", c2: "#11998e", c3: "#9be15d" },
    { name: "Cyber", c1: "#1a0f3d", c2: "#7b2ff7", c3: "#00f2fe" },
    { name: "Coffee", c1: "#1f130f", c2: "#6f4e37", c3: "#c7905b" },
    { name: "Lava", c1: "#3b0a0a", c2: "#ff416c", c3: "#ffb347" },
    { name: "Midnight", c1: "#0b1020", c2: "#232526", c3: "#6a82fb" },
    { name: "Berry", c1: "#2b0f54", c2: "#8e2de2", c3: "#ff4ecd" },
    { name: "Obsidian Rose", c1: "#0f0c29", c2: "#302b63", c3: "#ff416c" },
    { name: "Velvet Night", c1: "#1a1a2e", c2: "#16213e", c3: "#0f3460" },
    { name: "Deep Wine", c1: "#2b0f1c", c2: "#5a1e36", c3: "#a83279" },
    { name: "Silk Blue", c1: "#1f2a38", c2: "#2c3e50", c3: "#4ca1af" },
    { name: "Muted Gold", c1: "#232526", c2: "#6f625d", c3: "#b79891" },
    { name: "Ash Fade", c1: "#1f1c2c", c2: "#4a445f", c3: "#928dab" },
    { name: "Neon Pulse", c1: "#0f2027", c2: "#2c5364", c3: "#00f2fe" },
    { name: "Ultraviolet", c1: "#240b36", c2: "#6a1b75", c3: "#c31432" },
    { name: "Digital Aurora", c1: "#141e30", c2: "#243b55", c3: "#00c6ff" },
    { name: "Burnt Amber", c1: "#2c1b0f", c2: "#8b5e3c", c3: "#e6a756" },
    { name: "Graphite", c1: "#101214", c2: "#232526", c3: "#414345" },
    { name: "Steel Fade", c1: "#1f2731", c2: "#485563", c3: "#7d8fa1" }
  ];
  const walletStorageKey = "qr-studio-ultra.wallets";

  // Explicitly include Code 128, EAN, UPC, and ISBN formats for barcode scanning
  const scannerFormats = [
    Format.QRCode,      // QR codes (main functionality)
    Format.Code128,     // Code 128 barcodes
    Format.EAN13,       // EAN-13 (includes ISBN)
    Format.EAN8,        // EAN-8
    Format.UPC_A,       // UPC-A
    Format.UPC_E        // UPC-E
  ];

  onMount(() => {
    loadSavedWallets();
    loadSavedTemplates();
    loadGenerationHistory();
  });

  function applySolid(c: string) {
    color1 = c;
    fillType = "Solid";
    eyeOut = c;
    eyeIn = c;
    ringColor = c;
    frameTextColor = c;
    frameTextTopColor = c;
    centerOverlayColor = c;
  }
  function applyGradient(c1: string, c2: string, c3?: string) {
    color1 = c1;
    color2 = c2;
    color3 = c3 ?? c2;
    color4 = c3 ?? c2;
    useFourthStop = true;
    fillType = "Linear";
    eyeOut = c1;
    eyeIn = c2;
    ringColor = c1;
    ringColor2 = c2;
    ringColor3 = c3 ?? c2;
    ringColor4 = c3 ?? c2;
    ringUseFourthStop = true;
    ringColorMode = "gradient";
    frameTextColor = c1;
    frameTextTopColor = c1;
    centerOverlayColor = c1;
    centerOverlayColor2 = c2;
    centerOverlayColor3 = c3 ?? c2;
    centerOverlayColor4 = c3 ?? c2;
    centerOverlayUseFourthStop = true;
    centerOverlayColorMode = "gradient";
  }

  function getGradientPalette(
    c1: string,
    c2: string,
    c3?: string,
    c4?: string,
    useFourth = false
  ) {
    return {
      c1,
      c2,
      c3: c3 ?? c2,
      c4: useFourth ? (c4 ?? c3 ?? c2) : undefined
    };
  }

  function applyGradientStops(
    gradient: CanvasGradient,
    c1: string,
    c2: string,
    c3?: string,
    c4?: string,
    useFourth = false
  ) {
    gradient.addColorStop(0, c1);
    if (useFourth && c4) {
      gradient.addColorStop(0.33, c2);
      gradient.addColorStop(0.66, c3 ?? c2);
      gradient.addColorStop(1, c4);
    } else if (c3) {
      gradient.addColorStop(0.5, c2);
      gradient.addColorStop(1, c3);
    } else {
      gradient.addColorStop(1, c2);
    }
  }

  function gradientPreviewCss(c1: string, c2: string, c3?: string, c4?: string, useFourth = false) {
    return useFourth && c4
      ? `linear-gradient(135deg, ${c1} 0%, ${c2} 33%, ${c3 ?? c2} 66%, ${c4} 100%)`
      : c3
        ? `linear-gradient(135deg, ${c1} 0%, ${c2} 50%, ${c3} 100%)`
        : `linear-gradient(135deg, ${c1} 0%, ${c2} 100%)`;
  }

  function setActiveColorValue(value: string) {
    switch (activeColorTarget) {
      case "color1": color1 = value; break;
      case "color2": color2 = value; break;
      case "color3": color3 = value; break;
      case "color4": color4 = value; break;
      case "bgColor": bgColor = value; break;
      case "eyeOut": eyeOut = value; break;
      case "eyeIn": eyeIn = value; break;
      case "ringColor": ringColor = value; break;
      case "ringColor2": ringColor2 = value; break;
      case "ringColor3": ringColor3 = value; break;
      case "ringColor4": ringColor4 = value; break;
      case "frameTextColor": frameTextColor = value; break;
      case "centerOverlayColor": centerOverlayColor = value; break;
      case "centerOverlayColor2": centerOverlayColor2 = value; break;
      case "centerOverlayColor3": centerOverlayColor3 = value; break;
      case "centerOverlayColor4": centerOverlayColor4 = value; break;
    }
  }

  function getActiveColorValue() {
    switch (activeColorTarget) {
      case "color1": return color1;
      case "color2": return color2;
      case "color3": return color3;
      case "color4": return color4;
      case "bgColor": return bgColor;
      case "eyeOut": return eyeOut;
      case "eyeIn": return eyeIn;
      case "ringColor": return ringColor;
      case "ringColor2": return ringColor2;
      case "ringColor3": return ringColor3;
      case "ringColor4": return ringColor4;
      case "frameTextColor": return frameTextColor;
      case "centerOverlayColor": return centerOverlayColor;
      case "centerOverlayColor2": return centerOverlayColor2;
      case "centerOverlayColor3": return centerOverlayColor3;
      case "centerOverlayColor4": return centerOverlayColor4;
      default: return color1;
    }
  }

  function getColorValueByTarget(target: string) {
    switch (target) {
      case "color1": return color1;
      case "color2": return color2;
      case "color3": return color3;
      case "color4": return color4;
      case "bgColor": return bgColor;
      case "eyeOut": return eyeOut;
      case "eyeIn": return eyeIn;
      case "ringColor": return ringColor;
      case "ringColor2": return ringColor2;
      case "ringColor3": return ringColor3;
      case "ringColor4": return ringColor4;
      case "frameTextColor": return frameTextColor;
      case "centerOverlayColor": return centerOverlayColor;
      case "centerOverlayColor2": return centerOverlayColor2;
      case "centerOverlayColor3": return centerOverlayColor3;
      case "centerOverlayColor4": return centerOverlayColor4;
      default: return color1;
    }
  }

  function rememberColor(value: string) {
    recentColors = [value, ...recentColors.filter((entry) => entry !== value)].slice(0, 12);
  }

  function normalizeHexColor(value: string) {
    const raw = value.trim();
    if (!raw) return null;
    const normalized = raw.startsWith("#") ? raw : `#${raw}`;
    if (/^#[0-9a-fA-F]{6}$/.test(normalized)) return normalized.toUpperCase();
    if (/^#[0-9a-fA-F]{3}$/.test(normalized)) {
      const expanded = normalized
        .slice(1)
        .split("")
        .map((char) => `${char}${char}`)
        .join("");
      return `#${expanded}`.toUpperCase();
    }
    return null;
  }

  function updateColorTarget(target: string, value: string) {
    activeColorTarget = target;
    const normalized = normalizeHexColor(value);
    if (!normalized) return;
    setActiveColorValue(normalized);
    rememberColor(normalized);
  }

  function onStudioColorInput(event: Event) {
    const value = (event.currentTarget as HTMLInputElement).value;
    updateColorTarget(activeColorTarget, value);
  }

  function openNativePicker(target: string) {
    activeColorTarget = target;
    if (!fallbackColorInput) return;
    fallbackColorInput.value = normalizeHexColor(getActiveColorValue()) ?? "#000000";
    fallbackColorInput.click();
  }

  async function pickFromScreen(target = activeColorTarget) {
    const picker = (window as Window & { EyeDropper?: new () => { open: () => Promise<{ sRGBHex: string }> } }).EyeDropper;
    if (!picker) {
      openNativePicker(target);
      return;
    }

    try {
      const eyeDropper = new picker();
      const result = await eyeDropper.open();
      updateColorTarget(target, result.sRGBHex);
    } catch {
      openNativePicker(target);
    }
  }

  async function pickColorInto(target: string) {
    activeColorTarget = target;
    openNativePicker(target);
  }

  function applyColorChip(target: string, value: string) {
    updateColorTarget(target, value);
  }

  function onHexBlur(target: string, event: Event) {
    const input = event.currentTarget as HTMLInputElement;
    const normalized = normalizeHexColor(input.value);
    if (normalized) {
      updateColorTarget(target, normalized);
      input.value = normalized;
    } else {
      input.value = getColorValueByTarget(target);
      showSaveToastMessage("Enter a valid hex color like #1A73E8.", "error");
    }
  }

  function getRingPalette() {
    return ringGradientMode === "match-main"
      ? getGradientPalette(color1, color2, color3, color4, useFourthStop)
      : getGradientPalette(ringColor, ringColor2, ringColor3, ringColor4, ringUseFourthStop);
  }

  function getCenterOverlayPalette() {
    if (centerOverlayGradientMode === "match-main") {
      return getGradientPalette(color1, color2, color3, color4, useFourthStop);
    }
    if (centerOverlayGradientMode === "match-outer") {
      return getRingPalette();
    }
    return getGradientPalette(
      centerOverlayColor,
      centerOverlayColor2,
      centerOverlayColor3,
      centerOverlayColor4,
      centerOverlayUseFourthStop
    );
  }

  function triggerFileInput() { if (fileInput) fileInput.click(); }
  
  // --- SMART AUTO-CROPPER LOGIC ---
  function handleLogoUpload(e: any) {
    const file = e.target.files[0];
    if (!file) return;
    logoName = file.name;
    const reader = new FileReader();
    reader.onload = (ev: any) => {
      cropRawSrc = ev.target.result as string;
      showCropModal = true;
    };
    reader.readAsDataURL(file);
  }

  function onCropImgLoad() {
    if (!cropImgEl) return;
    cropImgNaturalW = cropImgEl.naturalWidth;
    cropImgNaturalH = cropImgEl.naturalHeight;
    cropZoom = 1.0;
    cropSize = Math.min(cropImgNaturalW, cropImgNaturalH);
    cropX = (cropImgNaturalW - cropSize) / 2;
    cropY = (cropImgNaturalH - cropSize) / 2;
  }

  function updateCropSize() {
    cropSize = Math.min(cropImgNaturalW, cropImgNaturalH) / cropZoom;
    cropX = Math.max(0, Math.min(cropX, cropImgNaturalW - cropSize));
    cropY = Math.max(0, Math.min(cropY, cropImgNaturalH - cropSize));
  }

  function onZoomChange(e: any) {
    cropZoom = parseFloat(e.target.value);
    updateCropSize();
  }

  function startDrag(e: any) {
    isDragging = true;
    const clientX = e.touches ? e.touches[0].clientX : e.clientX;
    const clientY = e.touches ? e.touches[0].clientY : e.clientY;
    dragStartX = clientX;
    dragStartY = clientY;
    cropStartX = cropX;
    cropStartY = cropY;
  }

  function onDrag(e: any) {
    if (!isDragging || !cropImgEl) return;
    const clientX = e.touches ? e.touches[0].clientX : e.clientX;
    const clientY = e.touches ? e.touches[0].clientY : e.clientY;
    const rect = cropImgEl.getBoundingClientRect();
    const scaleX = cropImgNaturalW / rect.width;
    const scaleY = cropImgNaturalH / rect.height;
    
    // Dragging logic inverted so dragging the image feels like panning a map
    const newX = cropStartX - (clientX - dragStartX) * scaleX;
    const newY = cropStartY - (clientY - dragStartY) * scaleY;
    
    cropX = Math.max(0, Math.min(newX, cropImgNaturalW - cropSize));
    cropY = Math.max(0, Math.min(newY, cropImgNaturalH - cropSize));
  }

  function stopDrag() { isDragging = false; }

  function trimTransparentCanvas(sourceCanvas: HTMLCanvasElement) {
    const ctx = sourceCanvas.getContext("2d");
    if (!ctx) return sourceCanvas;

    const { width, height } = sourceCanvas;
    const imageData = ctx.getImageData(0, 0, width, height).data;
    let minX = width;
    let minY = height;
    let maxX = -1;
    let maxY = -1;

    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const alpha = imageData[(y * width + x) * 4 + 3];
        if (alpha > 8) {
          minX = Math.min(minX, x);
          minY = Math.min(minY, y);
          maxX = Math.max(maxX, x);
          maxY = Math.max(maxY, y);
        }
      }
    }

    if (maxX < minX || maxY < minY) {
      logoTrimmedWidth = width;
      logoTrimmedHeight = height;
      return sourceCanvas;
    }

    const trimmedWidth = maxX - minX + 1;
    const trimmedHeight = maxY - minY + 1;
    const trimmedCanvas = document.createElement("canvas");
    trimmedCanvas.width = trimmedWidth;
    trimmedCanvas.height = trimmedHeight;
    const trimmedCtx = trimmedCanvas.getContext("2d");
    if (!trimmedCtx) return sourceCanvas;

    trimmedCtx.drawImage(sourceCanvas, minX, minY, trimmedWidth, trimmedHeight, 0, 0, trimmedWidth, trimmedHeight);
    logoTrimmedWidth = trimmedWidth;
    logoTrimmedHeight = trimmedHeight;
    return trimmedCanvas;
  }

  function getRenderedLogoDimensions(renderedQrSize: number) {
    const maxSize = renderedQrSize * (logoSizePercent / 100);
    const aspect = Math.max(logoTrimmedWidth, 1) / Math.max(logoTrimmedHeight, 1);

    if (aspect >= 1) {
      return {
        width: maxSize,
        height: maxSize / aspect
      };
    }

    return {
      width: maxSize * aspect,
      height: maxSize
    };
  }

  function getCenterOverlayDimensions(renderedQrSize: number) {
    const logoDimensions = getRenderedLogoDimensions(renderedQrSize);
    const minSide = Math.min(logoDimensions.width, logoDimensions.height);
    const lineWidth = Math.max(4, minSide * 0.04);
    const gap = Math.max(2, minSide * 0.03);

    return {
      width: logoDimensions.width + lineWidth + gap * 2,
      height: logoDimensions.height + lineWidth + gap * 2
    };
  }

  // CRITICAL FIX: Synchronous, double-decode prevention
  function commitCrop() {
    const canvas = document.createElement("canvas");
    canvas.width = 200;
    canvas.height = 200;
    const ctx = canvas.getContext("2d");
    if (!ctx || !cropImgEl) return;
    
    // Reuse the already-decoded cropImgEl element directly!
    ctx.drawImage(cropImgEl, cropX, cropY, cropSize, cropSize, 0, 0, 200, 200);

    const trimmedCanvas = trimTransparentCanvas(canvas);
    logoBase64 = trimmedCanvas.toDataURL("image/png");
    showCropModal = false;
  }

  function cancelCrop() {
    showCropModal = false;
    logoName = "";
    cropRawSrc = "";
    logoTrimmedWidth = 200;
    logoTrimmedHeight = 200;
  }

  function isNativeMobileDevice() {
    return (window as any).__TAURI_INTERNALS__ !== undefined &&
      (navigator.userAgent.includes('Android') || navigator.userAgent.includes('iPhone') || navigator.userAgent.includes('iPad'));
  }

  function showSaveToastMessage(message: string, tone: "success" | "error" | "info" = "success") {
    mobileSaveMessage = message;
    saveToastTone = tone;
    showSaveToast = false;
    if (saveToastTimer) clearTimeout(saveToastTimer);
    tick().then(() => {
      showSaveToast = true;
      saveToastTimer = setTimeout(() => {
        showSaveToast = false;
      }, 3500);
    });
  }

  function getErrorMessage(error: unknown) {
    return error instanceof Error ? error.message : String(error);
  }

  function colorWithAlpha(hex: string, alpha: number) {
    const { r, g, b } = hexToRgbParts(hex);
    return `rgba(${r}, ${g}, ${b}, ${alpha})`;
  }

  function chooseAiCenterGlyph(prompt: string, mood: string) {
    const text = `${prompt} ${mood}`.toLowerCase();
    if (/(moon|witch|night|lunar|silver)/.test(text)) return "☾";
    if (/(forest|nature|leaf|botanical|emerald|garden)/.test(text)) return "✣";
    if (/(luxury|gold|diamond|crystal|gem|marble)/.test(text)) return "◇";
    if (/(fire|lava|phoenix|volcanic|inferno)/.test(text)) return "✹";
    if (/(cyber|neon|holo|electric|future)/.test(text)) return "✦";
    if (/(space|cosmic|nebula|astral|star)/.test(text)) return "✧";
    return "✦";
  }

  function createAiCenterImage(prompt: string, mood: string, c1: string, c2: string, c3: string) {
    const canvas = document.createElement("canvas");
    canvas.width = 512;
    canvas.height = 512;
    const ctx = canvas.getContext("2d");
    if (!ctx) return "";

    ctx.clearRect(0, 0, 512, 512);

    const aura = ctx.createRadialGradient(256, 256, 20, 256, 256, 246);
    aura.addColorStop(0, colorWithAlpha(c2, 0.98));
    aura.addColorStop(0.42, colorWithAlpha(c1, 0.82));
    aura.addColorStop(0.78, colorWithAlpha(c3, 0.38));
    aura.addColorStop(1, "rgba(0, 0, 0, 0)");
    ctx.fillStyle = aura;
    ctx.beginPath();
    ctx.arc(256, 256, 246, 0, Math.PI * 2);
    ctx.fill();

    const core = ctx.createLinearGradient(92, 80, 420, 432);
    core.addColorStop(0, c1);
    core.addColorStop(0.52, c2);
    core.addColorStop(1, c3);
    ctx.save();
    ctx.shadowBlur = 34;
    ctx.shadowColor = colorWithAlpha(c2, 0.9);
    ctx.fillStyle = core;
    ctx.beginPath();
    ctx.arc(256, 256, 178, 0, Math.PI * 2);
    ctx.fill();
    ctx.restore();

    ctx.save();
    ctx.globalCompositeOperation = "screen";
    const highlight = ctx.createRadialGradient(190, 150, 8, 190, 150, 190);
    highlight.addColorStop(0, "rgba(255, 255, 255, 0.72)");
    highlight.addColorStop(0.4, colorWithAlpha(c3, 0.2));
    highlight.addColorStop(1, "rgba(255, 255, 255, 0)");
    ctx.fillStyle = highlight;
    ctx.beginPath();
    ctx.arc(210, 178, 190, 0, Math.PI * 2);
    ctx.fill();
    ctx.restore();

    ctx.save();
    ctx.strokeStyle = "rgba(255, 255, 255, 0.82)";
    ctx.lineWidth = 8;
    ctx.shadowBlur = 18;
    ctx.shadowColor = colorWithAlpha(c3, 0.85);
    ctx.beginPath();
    ctx.arc(256, 256, 184, 0, Math.PI * 2);
    ctx.stroke();
    ctx.lineWidth = 2.5;
    ctx.setLineDash([10, 14]);
    ctx.beginPath();
    ctx.arc(256, 256, 148, 0, Math.PI * 2);
    ctx.stroke();
    ctx.restore();

    const glyph = chooseAiCenterGlyph(prompt, mood);
    ctx.save();
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.font = "900 190px Georgia, 'Times New Roman', serif";
    ctx.shadowBlur = 22;
    ctx.shadowColor = "rgba(255, 255, 255, 0.95)";
    ctx.fillStyle = "rgba(255, 255, 255, 0.96)";
    ctx.fillText(glyph, 256, 252);
    ctx.restore();

    ctx.save();
    ctx.fillStyle = "rgba(255, 255, 255, 0.86)";
    for (let i = 0; i < 18; i += 1) {
      const seed = hashText(`${prompt}-${i}`);
      const angle = (seed % 628) / 100;
      const radius = 105 + (seed % 112);
      const x = 256 + Math.cos(angle) * radius;
      const y = 256 + Math.sin(angle) * radius;
      const size = 2 + (seed % 5);
      ctx.beginPath();
      ctx.arc(x, y, size, 0, Math.PI * 2);
      ctx.fill();
    }
    ctx.restore();

    return canvas.toDataURL("image/png");
  }

  function rememberRecentSave(label: string) {
    const timestamp = new Date().toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
    recentSaves = [{ label, timestamp }, ...recentSaves].slice(0, 3);
  }

  function toDisplayCase(value: string) {
    return value
      .replace(/_/g, " ")
      .replace(/\b\w/g, (letter) => letter.toUpperCase());
  }

  function getWalletScheme(type: string) {
    const schemes: Record<string, string> = {
      bitcoin: "bitcoin",
      ethereum: "ethereum",
      litecoin: "litecoin",
      dogecoin: "dogecoin",
      solana: "solana"
    };
    return schemes[type] ?? type;
  }

  function normalizeDecimalAmount(value: string) {
    const trimmed = value.trim();
    if (!trimmed) return "";
    if (!/^\d+(\.\d+)?$/.test(trimmed)) return trimmed;

    const normalized = trimmed
      .replace(/^0+(?=\d)/, "")
      .replace(/(\.\d*?)0+$/, "$1")
      .replace(/\.$/, "");

    return normalized || "0";
  }

  function decimalToAtomicUnits(value: string, decimals: number) {
    const trimmed = normalizeDecimalAmount(value);
    if (!trimmed || !/^\d+(\.\d+)?$/.test(trimmed)) return trimmed;

    const [whole, fraction = ""] = trimmed.split(".");
    const paddedFraction = (fraction + "0".repeat(decimals)).slice(0, decimals);
    const atomic = `${whole}${paddedFraction}`.replace(/^0+(?=\d)/, "");
    return atomic || "0";
  }

  function buildCryptoPayload() {
    const scheme = getWalletScheme(cryptoType);
    const address = cryptoAddr.trim();
    const amount = normalizeDecimalAmount(cryptoAmount);

    if (!address) return `${scheme}:`;

    if (cryptoType === "ethereum") {
      const query = new URLSearchParams();
      if (amount) {
        query.set("value", decimalToAtomicUnits(amount, 18));
      }
      if (cryptoMessage.trim()) {
        query.set("message", cryptoMessage.trim());
      }
      const suffix = query.toString() ? `?${query.toString()}` : "";
      return `ethereum:pay-${address}${suffix}`;
    }

    if (cryptoType === "solana") {
      const query = new URLSearchParams();
      if (amount) {
        query.set("amount", amount);
      }
      if (cryptoLabel.trim()) {
        query.set("label", cryptoLabel.trim());
      }
      if (cryptoMessage.trim()) {
        query.set("message", cryptoMessage.trim());
      }
      const suffix = query.toString() ? `?${query.toString()}` : "";
      return `solana:${address}${suffix}`;
    }

    const query = new URLSearchParams();
    if (amount) {
      query.set("amount", amount);
    }
    if (cryptoLabel.trim()) {
      query.set("label", cryptoLabel.trim());
    }
    if (cryptoMessage.trim()) {
      query.set("message", cryptoMessage.trim());
    }

    const suffix = query.toString() ? `?${query.toString()}` : "";
    return `${scheme}:${address}${suffix}`;
  }

  function loadSavedWallets() {
    if (typeof window === "undefined") return;
    try {
      const raw = window.localStorage.getItem(walletStorageKey);
      savedWallets = raw ? JSON.parse(raw) : [];
    } catch {
      savedWallets = [];
    }
  }

  function persistSavedWallets() {
    if (typeof window === "undefined") return;
    window.localStorage.setItem(walletStorageKey, JSON.stringify(savedWallets));
  }

  function saveCurrentWalletProfile() {
    if (!walletName.trim()) {
      showSaveToastMessage("Please enter a wallet profile name before saving.", "error");
      return;
    }
    if (!cryptoAddr.trim()) {
      showSaveToastMessage("Please enter a wallet address before saving.", "error");
      return;
    }

    const wallet: SavedWallet = {
      id: `${Date.now()}`,
      name: walletName.trim(),
      network: cryptoType,
      address: cryptoAddr.trim(),
      amount: cryptoAmount.trim(),
      label: cryptoLabel.trim(),
      message: cryptoMessage.trim()
    };

    savedWallets = [
      wallet,
      ...savedWallets.filter((entry) => !(entry.name === wallet.name && entry.address === wallet.address && entry.network === wallet.network))
    ];
    persistSavedWallets();
    showSaveToastMessage(`Saved wallet profile for ${wallet.name}.`, "success");
  }

  function loadWalletProfile(wallet: SavedWallet) {
    dataType = "Crypto";
    walletName = wallet.name;
    cryptoType = wallet.network;
    cryptoAddr = wallet.address;
    cryptoAmount = wallet.amount;
    cryptoLabel = wallet.label;
    cryptoMessage = wallet.message;
    showSaveToastMessage(`Loaded ${wallet.name}.`, "info");
  }

  function deleteWalletProfile(id: string) {
    savedWallets = savedWallets.filter((wallet) => wallet.id !== id);
    persistSavedWallets();
    showSaveToastMessage("Wallet profile removed.", "info");
  }

  function getStyleTemplateSnapshot(options: { includeBatch?: boolean } = {}) {
    const snapshot: Record<string, any> = {
      dataType, qrData, wifiSsid, wifiPass,
      petName, microchipNum, ownerName, ownerPhone, ownerAddr,
      vCardFirst, vCardLast, vCardOrg, vCardPhone, vCardEmail,
      emailTo, emailSub, emailBody, smsPhone, smsMsg, phoneNum, geoLat, geoLng,
      waPhone, waMsg, cryptoAddr, cryptoType, cryptoAmount, cryptoLabel, cryptoMessage,
      walletName, eventTitle, eventStart, eventEnd, eventLoc, socialUser, socialPlatform,
      zoomMeetingId, zoomPass, linkedinUser, ytHandle, tiktokUser,
      bgShape, mainShape, fillType, color1, color2, color3, color4, useFourthStop, bgColor,
      eyeShape, eyeOut, eyeIn, enableFrame, frameText, frameTextTop, frameTextMode,
      frameTextTopMode, frameTextRadius, frameTextTopRadius, frameTextSpacing,
      frameTextTopSpacing, frameTextSize, frameTextTopSize, frameTextColor,
      frameTextTopColor, matchTextStyle, matchTextTopStyle, transparentTextBg,
      transparentTextTopBg, ringStyle, ringColor, ringColor2, ringColor3, ringColor4,
      ringUseFourthStop, ringGradientMode, ringColorMode, transparentFrameBg,
      centerOverlayMode, centerOverlayStyle, centerOverlayColor, centerOverlayColor2,
      centerOverlayColor3, centerOverlayColor4, centerOverlayUseFourthStop,
      centerOverlayGradientMode, centerOverlayColorMode, logoName, logoBase64,
      logoTrimmedWidth, logoTrimmedHeight, logoSizePercent, logoOpacityPercent
    };

    if (options.includeBatch) {
      snapshot.batchInput = batchInput;
    }

    return snapshot;
  }

  function applyStyleTemplate(settings: Record<string, any> | undefined) {
    console.log("applyStyleTemplate called with settings:", settings);
    if (!settings || typeof settings !== "object") {
      showSaveToastMessage("This template could not be loaded.", "error");
      return false;
    }

    if (settings.batchInput) {
      batchInput = settings.batchInput;
    } else {
      batchInput = "";
    }
    batchResults = [];

    dataType = settings.dataType ?? dataType;
    qrData = settings.qrData ?? qrData;
    wifiSsid = settings.wifiSsid ?? wifiSsid;
    wifiPass = settings.wifiPass ?? wifiPass;
    petName = settings.petName ?? petName;
    microchipNum = settings.microchipNum ?? microchipNum;
    ownerName = settings.ownerName ?? ownerName;
    ownerPhone = settings.ownerPhone ?? ownerPhone;
    ownerAddr = settings.ownerAddr ?? ownerAddr;
    vCardFirst = settings.vCardFirst ?? vCardFirst;
    vCardLast = settings.vCardLast ?? vCardLast;
    vCardOrg = settings.vCardOrg ?? vCardOrg;
    vCardPhone = settings.vCardPhone ?? vCardPhone;
    vCardEmail = settings.vCardEmail ?? vCardEmail;
    emailTo = settings.emailTo ?? emailTo;
    emailSub = settings.emailSub ?? emailSub;
    emailBody = settings.emailBody ?? emailBody;
    smsPhone = settings.smsPhone ?? smsPhone;
    smsMsg = settings.smsMsg ?? smsMsg;
    phoneNum = settings.phoneNum ?? phoneNum;
    geoLat = settings.geoLat ?? geoLat;
    geoLng = settings.geoLng ?? geoLng;
    waPhone = settings.waPhone ?? waPhone;
    waMsg = settings.waMsg ?? waMsg;
    cryptoAddr = settings.cryptoAddr ?? cryptoAddr;
    cryptoType = settings.cryptoType ?? cryptoType;
    cryptoAmount = settings.cryptoAmount ?? cryptoAmount;
    cryptoLabel = settings.cryptoLabel ?? cryptoLabel;
    cryptoMessage = settings.cryptoMessage ?? cryptoMessage;
    walletName = settings.walletName ?? walletName;
    eventTitle = settings.eventTitle ?? eventTitle;
    eventStart = settings.eventStart ?? eventStart;
    eventEnd = settings.eventEnd ?? eventEnd;
    eventLoc = settings.eventLoc ?? eventLoc;
    socialUser = settings.socialUser ?? socialUser;
    socialPlatform = settings.socialPlatform ?? socialPlatform;
    zoomMeetingId = settings.zoomMeetingId ?? zoomMeetingId;
    zoomPass = settings.zoomPass ?? zoomPass;
    linkedinUser = settings.linkedinUser ?? linkedinUser;
    ytHandle = settings.ytHandle ?? ytHandle;
    tiktokUser = settings.tiktokUser ?? tiktokUser;
    bgShape = settings.bgShape ?? bgShape;
    mainShape = settings.mainShape ?? mainShape;
    fillType = settings.fillType ?? fillType;
    color1 = settings.color1 ?? color1;
    color2 = settings.color2 ?? color2;
    color3 = settings.color3 ?? color3;
    color4 = settings.color4 ?? color4;
    useFourthStop = settings.useFourthStop ?? useFourthStop;
    bgColor = settings.bgColor ?? bgColor;
    eyeShape = settings.eyeShape ?? eyeShape;
    eyeOut = settings.eyeOut ?? eyeOut;
    eyeIn = settings.eyeIn ?? eyeIn;
    enableFrame = settings.enableFrame ?? enableFrame;
    frameText = settings.frameText ?? frameText;
    frameTextTop = settings.frameTextTop ?? frameTextTop;
    frameTextMode = settings.frameTextMode ?? frameTextMode;
    frameTextTopMode = settings.frameTextTopMode ?? frameTextTopMode;
    frameTextRadius = settings.frameTextRadius ?? frameTextRadius;
    frameTextTopRadius = settings.frameTextTopRadius ?? frameTextTopRadius;
    frameTextSpacing = settings.frameTextSpacing ?? frameTextSpacing;
    frameTextTopSpacing = settings.frameTextTopSpacing ?? frameTextTopSpacing;
    frameTextSize = settings.frameTextSize ?? frameTextSize;
    frameTextTopSize = settings.frameTextTopSize ?? frameTextTopSize;
    frameTextColor = settings.frameTextColor ?? frameTextColor;
    frameTextTopColor = settings.frameTextTopColor ?? frameTextTopColor;
    matchTextStyle = settings.matchTextStyle ?? matchTextStyle;
    matchTextTopStyle = settings.matchTextTopStyle ?? matchTextTopStyle;
    transparentTextBg = settings.transparentTextBg ?? transparentTextBg;
    transparentTextTopBg = settings.transparentTextTopBg ?? transparentTextTopBg;
    ringStyle = settings.ringStyle ?? ringStyle;
    ringColor = settings.ringColor ?? ringColor;
    ringColor2 = settings.ringColor2 ?? ringColor2;
    ringColor3 = settings.ringColor3 ?? ringColor3;
    ringColor4 = settings.ringColor4 ?? ringColor4;
    ringUseFourthStop = settings.ringUseFourthStop ?? ringUseFourthStop;
    ringGradientMode = settings.ringGradientMode ?? ringGradientMode;
    ringColorMode = settings.ringColorMode ?? ringColorMode;
    transparentFrameBg = settings.transparentFrameBg ?? transparentFrameBg;
    centerOverlayMode = settings.centerOverlayMode ?? centerOverlayMode;
    centerOverlayStyle = settings.centerOverlayStyle ?? centerOverlayStyle;
    centerOverlayColor = settings.centerOverlayColor ?? centerOverlayColor;
    centerOverlayColor2 = settings.centerOverlayColor2 ?? centerOverlayColor2;
    centerOverlayColor3 = settings.centerOverlayColor3 ?? centerOverlayColor3;
    centerOverlayColor4 = settings.centerOverlayColor4 ?? centerOverlayColor4;
    centerOverlayUseFourthStop = settings.centerOverlayUseFourthStop ?? centerOverlayUseFourthStop;
    centerOverlayGradientMode = settings.centerOverlayGradientMode ?? centerOverlayGradientMode;
    centerOverlayColorMode = settings.centerOverlayColorMode ?? centerOverlayColorMode;
    logoName = settings.logoName ?? logoName;
    logoBase64 = settings.logoBase64 ?? logoBase64;
    logoTrimmedWidth = settings.logoTrimmedWidth ?? logoTrimmedWidth;
    logoTrimmedHeight = settings.logoTrimmedHeight ?? logoTrimmedHeight;
    logoSizePercent = settings.logoSizePercent ?? logoSizePercent;
    logoOpacityPercent = settings.logoOpacityPercent ?? logoOpacityPercent;
    if (typeof settings.batchInput === "string") {
      batchInput = settings.batchInput;
      batchResults = [];
    }
    return true;
  }

  function loadSavedTemplates() {
    if (typeof window === "undefined") return;
    try {
      const raw = window.localStorage.getItem(templateStorageKey);
      savedTemplates = raw ? JSON.parse(raw) : [];
    } catch {
      savedTemplates = [];
    }
  }

  function persistSavedTemplates() {
    if (typeof window === "undefined") return;
    window.localStorage.setItem(templateStorageKey, JSON.stringify(savedTemplates));
  }

  function getTemplateBatchLines(template: StudioTemplate) {
    const savedBatchInput = typeof template.settings?.batchInput === "string" ? template.settings.batchInput : "";
    return savedBatchInput
      .split(/\r?\n/)
      .map((line) => line.trim())
      .filter(Boolean)
      .slice(0, 50);
  }

  function saveTemplate(kind: "single" | "batch") {
    console.log("saveTemplate called, kind:", kind);
    if (kind === "batch" && !getBatchLines().length) {
      showSaveToastMessage("Add one batch payload per line before saving a batch template.", "error");
      return;
    }

    const fallbackName = kind === "batch" ? `Batch ${getBatchLines().length} items` : `${fillType} ${mainShape} template`;
    const name = (templateName.trim() || fallbackName).slice(0, 48);
    const template: StudioTemplate = {
      id: `${Date.now()}`,
      name,
      kind,
      createdAt: new Date().toLocaleDateString([], { day: "2-digit", month: "2-digit", year: "numeric" }),
      settings: getStyleTemplateSnapshot({ includeBatch: kind === "batch" })
    };
    const nextTemplates = [template, ...savedTemplates.filter((item) => item.name !== name)].slice(0, 12);

    try {
      savedTemplates = nextTemplates;
      persistSavedTemplates();
      templateName = "";
      showSaveToastMessage(`${kind === "batch" ? "Batch template" : "Template"} saved: ${name}.`, "success");
    } catch {
      const slimTemplate: StudioTemplate = {
        ...template,
        settings: { ...template.settings, logoBase64: "", logoName: "" }
      };
      try {
        savedTemplates = [slimTemplate, ...savedTemplates.filter((item) => item.name !== name)].slice(0, 12);
        persistSavedTemplates();
        templateName = "";
        showSaveToastMessage(`${kind === "batch" ? "Batch template" : "Template"} saved without the center logo because storage is full.`, "info");
      } catch {
        savedTemplates = savedTemplates.filter((item) => item.id !== template.id);
        showSaveToastMessage("Template could not be saved because browser storage is full.", "error");
      }
    }
  }

  function saveCurrentTemplate() {
    saveTemplate("single");
  }

  function saveCurrentBatchTemplate() {
    saveTemplate("batch");
  }

  async function loadTemplate(template: StudioTemplate) {
    console.log("loadTemplate called:", template.name, "kind:", template.kind);
    const loaded = applyStyleTemplate(template.settings);
    if (!loaded) return;

    // Ensure batchInput is set if it exists in settings
    if (template.settings?.batchInput) {
      batchInput = template.settings.batchInput;
    }

    const isBatchTemplate = template.kind === "batch" || getTemplateBatchLines(template).length > 0;

    if (generationTimer) {
      clearTimeout(generationTimer);
      generationTimer = null;
    }

    await tick();
    if (isBatchTemplate) {
      await generateBatch();
    } else if (buildFinalQrData().trim()) {
      runGeneration();
    }
    showSaveToastMessage(`Template loaded: ${template.name}.`, "info");
  }

  function deleteTemplate(id: string) {
    console.log("deleteTemplate called, id:", id);
    savedTemplates = savedTemplates.filter((item) => item.id !== id);
    persistSavedTemplates();
    showSaveToastMessage("Template removed.", "info");
  }

  function loadGenerationHistory() {
    if (typeof window === "undefined") return;
    try {
      const raw = window.localStorage.getItem(historyStorageKey);
      generationHistory = raw ? JSON.parse(raw) : [];
    } catch {
      generationHistory = [];
    }
  }

  function persistGenerationHistory() {
    if (typeof window === "undefined") return;
    window.localStorage.setItem(historyStorageKey, JSON.stringify(generationHistory));
  }

  function rememberGeneration(payload: string, label: string) {
    if (!payload.trim()) return;
    const score = getScannabilityScore(payload).score;
    const entry: HistoryEntry = {
      id: `${Date.now()}`,
      label: label.slice(0, 64),
      dataType,
      payload,
      score,
      createdAt: new Date().toLocaleString([], { day: "2-digit", month: "2-digit", year: "numeric", hour: "2-digit", minute: "2-digit" })
    };
    generationHistory = [entry, ...generationHistory.filter((item) => item.payload !== payload)].slice(0, 20);
    persistGenerationHistory();
  }

  function loadHistoryEntry(entry: HistoryEntry) {
    dataType = "Text";
    qrData = entry.payload;
    showSaveToastMessage(`Loaded history item: ${entry.label}.`, "info");
    manualGenerationRequested = true;
    runGeneration();
  }

  function clearHistory() {
    generationHistory = [];
    persistGenerationHistory();
    showSaveToastMessage("History cleared.", "info");
  }

  function hexToRgbParts(value: string) {
    const clean = value.trim().replace("#", "");
    const expanded = clean.length === 3 ? clean.split("").map((ch) => ch + ch).join("") : clean;
    if (!/^[0-9a-f]{6}$/i.test(expanded)) return { r: 0, g: 0, b: 0 };
    return {
      r: parseInt(expanded.slice(0, 2), 16),
      g: parseInt(expanded.slice(2, 4), 16),
      b: parseInt(expanded.slice(4, 6), 16)
    };
  }

  function relativeLuminance(hex: string) {
    const { r, g, b } = hexToRgbParts(hex);
    const channel = (n: number) => {
      const c = n / 255;
      return c <= 0.03928 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);
    };
    return 0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b);
  }

  function contrastRatio(foreground: string, background: string) {
    const fg = relativeLuminance(foreground);
    const bg = relativeLuminance(background);
    const light = Math.max(fg, bg);
    const dark = Math.min(fg, bg);
    return (light + 0.05) / (dark + 0.05);
  }

  function getScannabilityScore(payload: string, _trigger?: string) {
    const notes: string[] = [];
    const trimmed = payload.trim();
    if (!trimmed) return { score: 0, label: "Waiting", className: "empty", notes: ["Add content to score scannability."] };

    let score = 100;
    if (trimmed.length > 120) { score -= 8; notes.push("Payload is getting dense."); }
    if (trimmed.length > 300) { score -= 14; notes.push("Long payload may need larger print size."); }
    if (trimmed.length > 700) { score -= 18; notes.push("Very long payload reduces scan margin."); }

    const foregroundColors = [color1];
    if (fillType !== "Solid") {
      foregroundColors.push(color2, color3);
      if (useFourthStop) foregroundColors.push(color4);
    }
    const contrast = Math.min(...foregroundColors.map((fg) => contrastRatio(fg, bgColor)));
    if (contrast < 3) { score -= 28; notes.push("Low foreground/background contrast."); }
    else if (contrast < 4.5) { score -= 14; notes.push("Contrast is usable but a little tight."); }

    if (logoBase64 && logoSizePercent > 24) { score -= 10; notes.push("Center logo is above the safest size."); }
    if (logoBase64 && logoSizePercent > 30) { score -= 14; notes.push("Large logo may block data modules."); }
    if (enableFrame && transparentFrameBg) { score -= 6; notes.push("Transparent frame background depends on the surface behind it."); }
    if (mainShape !== "square") { score -= 4; notes.push("Styled modules are slightly harder to scan than square modules."); }
    if (fillType !== "Solid") { score -= 5; notes.push("Gradients can reduce module contrast on some scanners."); }

    score = Math.max(10, Math.min(100, Math.round(score)));
    if (!notes.length) notes.push("Strong contrast and layout for scanning.");
    if (score >= 86) return { score, label: "Excellent", className: "strong", notes };
    if (score >= 70) return { score, label: "Good", className: "good", notes };
    if (score >= 52) return { score, label: "Caution", className: "warning", notes };
    return { score, label: "Risky", className: "danger", notes };
  }

  function getBatchLines() {
    return batchInput
      .split(/\r?\n/)
      .map((line) => line.trim())
      .filter(Boolean)
      .slice(0, 50);
  }

  function loadImageSource(src: string) {
    return new Promise<HTMLImageElement>((resolve, reject) => {
      const img = new Image();
      img.onload = () => resolve(img);
      img.onerror = () => {
        console.error("loadImageSource error for src:", src.slice(0, 100) + "...");
        reject(new Error("Generated batch image could not be loaded."));
      };
      img.src = src;
    });
  }

  function drawCenteredLogoOverlay(c: CanvasRenderingContext2D, centerX: number, centerY: number, renderedQrSize: number) {
    drawGlobalCenterOverlay(c, centerX, centerY, renderedQrSize);
  }

  function drawBatchFrameText(c: CanvasRenderingContext2D, text: string, isTop: boolean, cx = 400, cy = 400) {
    drawFrameText(c, text, isTop, cx, cy);
  }

  async function renderBatchImage(payload: string) {
    console.log("renderBatchImage for payload:", payload);
    const options = buildQrRenderOptions(payload);
    console.log("Render options built:", options);
    const baseImage = await invoke<string>("generate_ultra_qr", {
      options
    });
    console.log("Backend generated image, length:", baseImage.length);
    const img = await loadImageSource(baseImage);
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d");
    if (!ctx) throw new Error("Batch canvas context could not be created.");

    if (enableFrame) {
      canvas.width = 800;
      canvas.height = 800;
      ctx.save();
      drawShapePath(ctx, bgShape, 400, 400, 800);
      ctx.clip();
      if (!transparentFrameBg) {
        ctx.fillStyle = bgColor;
        ctx.fillRect(0, 0, 800, 800);
      }
      drawRingDecoration(
        ctx,
        ringStyle,
        bgShape,
        400,
        400,
        700,
        ringColor,
        getRingPalette(),
        ringStyle !== "solid" && ringStyle !== "none" && ringColorMode === "gradient"
      );

      let qrSize = 440;
      if (bgShape === "diamond") qrSize = 340;
      else if (bgShape === "octagon") qrSize = 400;
      else if (bgShape === "rounded") qrSize = 460;
      else if (bgShape === "square") qrSize = 480;
      const qrOffset = (800 - qrSize) / 2;

      ctx.save();
      drawShapePath(ctx, bgShape, 400, 400, 660);
      ctx.clip();
      ctx.drawImage(img, qrOffset, qrOffset, qrSize, qrSize);
      drawCenteredLogoOverlay(ctx, 400, 400, qrSize);
      ctx.restore();

      drawBatchFrameText(ctx, frameTextTop, true);
      drawBatchFrameText(ctx, frameText, false);
      ctx.restore();
    } else {
      canvas.width = 600;
      canvas.height = 600;
      ctx.save();
      drawShapePath(ctx, bgShape, 300, 300, 600);
      ctx.clip();
      ctx.fillStyle = bgColor;
      ctx.fillRect(0, 0, 600, 600);

      let qrSize = 600;
      if (bgShape === "circle") qrSize = 400;
      else if (bgShape === "rounded") qrSize = 580;
      else if (bgShape === "diamond") qrSize = 280;
      else if (bgShape === "octagon") qrSize = 480;
      const qrOffset = (600 - qrSize) / 2;

      ctx.drawImage(img, qrOffset, qrOffset, qrSize, qrSize);
      drawCenteredLogoOverlay(ctx, 300, 300, qrSize);
      ctx.restore();
    }

    return canvas.toDataURL("image/png");
  }

  async function generateBatch() {
    console.log("generateBatch called");
    const lines = getBatchLines();
    console.log("Batch lines:", lines);
    if (!lines.length) {
      showSaveToastMessage("Add one batch payload per line.", "error");
      return;
    }

    batchBusy = true;
    batchResults = [];
    batchProgress = 0;
    batchTotal = lines.length;
    try {
      const results: BatchResult[] = [];
      for (const [index, payload] of lines.entries()) {
        console.log(`Rendering batch item ${index + 1}/${lines.length}: ${payload}`);
        const image = await renderBatchImage(payload);
        results.push({
          id: `${Date.now()}-${index}`,
          label: `Batch ${index + 1}`,
          payload,
          image,
          score: getScannabilityScore(payload).score
        });
        batchProgress = index + 1;
      }
      batchResults = results;
      console.log("Batch results generated:", batchResults.length);
      showSaveToastMessage(`Generated ${results.length} batch QR codes.`, "success");
    } catch (e) {
      console.error("Batch generation failed:", e);
      showSaveToastMessage("Batch generation failed: " + e, "error");
    } finally {
      batchBusy = false;
      batchProgress = 0;
      batchTotal = 0;
    }
  }

  function loadBatchResult(result: BatchResult) {
    console.log("loadBatchResult called for:", result.label);
    const payload = result.payload.trim();
    const kind = classifyScan(payload, Format.QRCode);
    
    // Default fallback
    qrData = payload;

    if (kind === "Link") {
      dataType = "URL";
    } else if (kind === "WiFi") {
      dataType = "WiFi";
      const ssidMatch = payload.match(/S:([^;]+)/i);
      const passMatch = payload.match(/P:([^;]+)/i);
      if (ssidMatch) wifiSsid = ssidMatch[1];
      if (passMatch) wifiPass = passMatch[1];
    } else if (kind === "Contact") {
      dataType = "vCard";
      const fnMatch = payload.match(/FN:([^\n]+)/i);
      const orgMatch = payload.match(/ORG:([^\n]+)/i);
      const telMatch = payload.match(/TEL:([^\n]+)/i);
      const emailMatch = payload.match(/EMAIL:([^\n]+)/i);
      if (fnMatch) {
        const parts = fnMatch[1].split(" ");
        vCardFirst = parts[0] || "";
        vCardLast = parts.slice(1).join(" ") || "";
      }
      if (orgMatch) vCardOrg = orgMatch[1];
      if (telMatch) vCardPhone = telMatch[1];
      if (emailMatch) vCardEmail = emailMatch[1];
    } else if (kind === "Event") {
      dataType = "Event";
      const sumMatch = payload.match(/SUMMARY:([^\n]+)/i);
      const locMatch = payload.match(/LOCATION:([^\n]+)/i);
      if (sumMatch) eventTitle = sumMatch[1];
      if (locMatch) eventLoc = locMatch[1];
    } else if (payload.startsWith("mailto:")) {
      dataType = "Email";
      const to = payload.split(":")[1]?.split("?")[0] || "";
      emailTo = to;
    } else if (payload.startsWith("tel:")) {
      dataType = "Phone";
      phoneNum = payload.split(":")[1] || "";
    } else if (payload.startsWith("smsto:")) {
      dataType = "SMS";
      const parts = payload.split(":");
      smsPhone = parts[1] || "";
      smsMsg = parts.slice(2).join(":") || "";
    } else if (payload.startsWith("geo:")) {
      dataType = "Geo";
      const coords = payload.split(":")[1]?.split(",") || [];
      geoLat = coords[0] || "";
      geoLng = coords[1] || "";
    } else if (kind === "Crypto Wallet") {
      dataType = "Crypto";
      const parts = payload.split(":");
      cryptoType = parts[0] || "bitcoin";
      cryptoAddr = parts[1]?.split("?")[0] || "";
    } else {
      dataType = "Text";
    }

    manualGenerationRequested = true;
    runGeneration();
    showSaveToastMessage(`Loaded ${result.label} into Studio.`, "info");
  }

  function strip_data_url_prefix(b64: string) {
    if (!b64) return "";
    const parts = b64.split(",");
    return parts.length > 1 ? parts[1] : parts[0];
  }

  function clearBatchResults() {
    batchResults = [];
    showSaveToastMessage("Batch results cleared.", "info");
  }

  async function saveBatchResult(result: BatchResult) {
    console.log("saveBatchResult called for:", result.label);
    try {
      const isMobile = isNativeMobileDevice();
      const b64Data = result.image; // Keep full data URL for invoke
      
      if (isMobile) {
        const msg = await invoke<{ message: string }>("save_to_device", { b64: b64Data, format: "png" });
        showSaveToastMessage(msg.message, "success");
      } else {
        const filePath = await save({
          filters: [{ name: 'Image', extensions: ['png'] }],
          defaultPath: `Batch_${result.label.replace(/\s+/g, '_')}_${Date.now()}.png`
        });
        if (filePath) {
          const msg = await invoke("save_to_path", { b64: b64Data, path: filePath });
          showSaveToastMessage(String(msg), "success");
        }
      }
    } catch (e) {
      showSaveToastMessage("Failed to save batch item: " + e, "error");
    }
  }

  async function saveAllBatchAsZip() {
    console.log("saveAllBatchAsZip called, results count:", batchResults.length);
    if (!batchResults.length) return;
    try {
      const isMobile = isNativeMobileDevice();
      if (isMobile) {
        showSaveToastMessage("ZIP export is not supported on mobile yet.", "error");
        return;
      }

      const filePath = await save({
        filters: [{ name: 'ZIP Archive', extensions: ['zip'] }],
        defaultPath: `Batch_Export_${Date.now()}.zip`
      });

      if (filePath) {
        batchBusy = true;
        const msg = await invoke("save_batch_as_zip", { items: batchResults, path: filePath });
        showSaveToastMessage(String(msg), "success");
      }
    } catch (e) {
      showSaveToastMessage("Failed to export ZIP: " + e, "error");
    } finally {
      batchBusy = false;
    }
  }

  function classifyScan(content: string, format: Format) {
    if (format !== Format.QRCode) return "Barcode";
    if (isEncryptedQrPayload(content)) return "Encrypted QR";
    if (/^(https?:\/\/|www\.)/i.test(content)) return "Link";
    if (/^(bitcoin|ethereum|litecoin|dogecoin|solana):/i.test(content)) return "Crypto Wallet";
    if (/^(mailto:|tel:|sms:|smsto:|geo:)/i.test(content)) return "Action";
    if (/^WIFI:/i.test(content)) return "WiFi";
    if (/^BEGIN:VCARD/i.test(content)) return "Contact";
    if (/^BEGIN:VCALENDAR/i.test(content)) return "Event";
    return "QR Code";
  }

  function canOpenResult(value: string) {
    return /^(https?:\/\/|www\.|mailto:|tel:|sms:|smsto:|geo:)/i.test(value.trim());
  }

  function isEncryptedQrPayload(value: string) {
    return value.trim().startsWith("QRU1:");
  }

  function useEncryptedPayloadForQr(payload: string) {
    dataType = "Text";
    qrData = payload.trim();
    runGeneration();
  }

  function getForensicDateStamp() {
    const now = new Date();
    const day = String(now.getDate()).padStart(2, "0");
    const month = String(now.getMonth() + 1).padStart(2, "0");
    const year = now.getFullYear();
    return `${day}/${month}/${year}`;
  }

  function getPayloadForensicDate(payload: string) {
    try {
      const encoded = payload.trim().replace(/^QRU1:/, "");
      const normalized = encoded.replace(/-/g, "+").replace(/_/g, "/");
      const padded = normalized.padEnd(normalized.length + (4 - normalized.length % 4) % 4, "=");
      const decoded = atob(padded);
      const parsed = JSON.parse(decoded);
      return typeof parsed.created_date === "string" ? parsed.created_date : "";
    } catch {
      return "";
    }
  }

  function getPayloadAlgorithmLabel(payload: string) {
    try {
      const encoded = payload.trim().replace(/^QRU1:/, "");
      const normalized = encoded.replace(/-/g, "+").replace(/_/g, "/");
      const padded = normalized.padEnd(normalized.length + (4 - normalized.length % 4) % 4, "=");
      const decoded = atob(padded);
      const parsed = JSON.parse(decoded);
      if (parsed.algorithm === "aes-256-gcm") return "AES-256-GCM";
      if (parsed.algorithm === "chacha20-poly1305") return "ChaCha20-Poly1305";
      return typeof parsed.algorithm === "string" ? parsed.algorithm : "";
    } catch {
      return "";
    }
  }

  function hideScanDecryptSuccessBanner() {
    showScanDecryptSuccessBanner = false;
    if (scanDecryptSuccessTimer) {
      clearTimeout(scanDecryptSuccessTimer);
      scanDecryptSuccessTimer = null;
    }
  }

  function showTemporaryScanDecryptSuccessBanner() {
    showScanDecryptSuccessBanner = true;
    if (scanDecryptSuccessTimer) clearTimeout(scanDecryptSuccessTimer);
    scanDecryptSuccessTimer = setTimeout(() => {
      showScanDecryptSuccessBanner = false;
      scanDecryptSuccessTimer = null;
    }, 4200);
  }

  function loadScannedEncryptedPayload() {
    decryptPayload = scannedResult.trim();
    decryptedPlaintext = "";
    decryptedForensicDate = "";
    handleEncryptedQrScan(scannedResult);
  }

  function handleEncryptedQrScan(payload: string) {
    scanEncryptedPayload = payload.trim();
    scanDecryptPassphrase = "";
    scanDecryptedPlaintext = "";
    scanDecryptedForensicDate = "";
    hideScanDecryptSuccessBanner();
    scanDecryptError = "";
    showScanDecryptPassphrase = false;
    showScanDecryptModal = true;
    decryptPayload = scanEncryptedPayload;
    decryptedPlaintext = "";
    decryptedForensicDate = "";
  }

  function closeScanDecryptModal() {
    showScanDecryptModal = false;
    scanEncryptedPayload = "";
    scanDecryptPassphrase = "";
    scanDecryptedPlaintext = "";
    scanDecryptedForensicDate = "";
    hideScanDecryptSuccessBanner();
    scanDecryptError = "";
    showScanDecryptPassphrase = false;
  }

  function retryScanDecryption() {
    scanDecryptPassphrase = "";
    scanDecryptedPlaintext = "";
    scanDecryptedForensicDate = "";
    hideScanDecryptSuccessBanner();
    scanDecryptError = "";
    showScanDecryptPassphrase = false;
  }

  async function decryptScannedEncryptedQr() {
    if (!scanDecryptPassphrase) {
      scanDecryptError = "Enter the passphrase for this encrypted QR.";
      return;
    }

    cryptoBusy = true;
    scanDecryptError = "";
    scanDecryptedPlaintext = "";
    scanDecryptedForensicDate = "";
    hideScanDecryptSuccessBanner();
    try {
      scanDecryptedPlaintext = await invoke<string>("decrypt_qr_payload", {
        encryptedPayload: scanEncryptedPayload,
        passphrase: scanDecryptPassphrase
      });
      scanDecryptedForensicDate = getPayloadForensicDate(scanEncryptedPayload);
      showTemporaryScanDecryptSuccessBanner();
      scannedResult = scanDecryptedPlaintext;
      scannedKind = canOpenResult(scanDecryptedPlaintext) ? "Decrypted Link" : "Decrypted Text";
      decryptPayload = scanEncryptedPayload;
      decryptedPlaintext = scanDecryptedPlaintext;
      decryptedForensicDate = scanDecryptedForensicDate;
      showSaveToastMessage(
        scanDecryptedForensicDate
          ? `Encrypted QR verified. Forensic date: ${scanDecryptedForensicDate}.`
          : "Encrypted QR decrypted locally. No forensic date found.",
        "success"
      );
    } catch (e) {
      scanDecryptError = "Could not decrypt. Check the passphrase and QR payload.";
    } finally {
      cryptoBusy = false;
    }
  }

  async function encryptPlaintextForQr() {
    if (!encryptPlaintext.trim()) {
      showSaveToastMessage("Enter plaintext before encrypting.", "error");
      return;
    }
    if (!encryptPassphrase) {
      showSaveToastMessage("Enter a passphrase before encrypting.", "error");
      return;
    }

    cryptoBusy = true;
    encryptedPayload = "";
    encryptedForensicDate = getForensicDateStamp();
    try {
      encryptedPayload = await invoke<string>("encrypt_qr_payload", {
        plaintext: encryptPlaintext,
        passphrase: encryptPassphrase,
        encryptionAlgorithm,
        forensicDate: encryptedForensicDate
      });
      useEncryptedPayloadForQr(encryptedPayload);
      showSaveToastMessage(`Encrypted payload dated ${encryptedForensicDate}.`, "success");
      
      // CONFETTI / SUCCESS GLOW
      triggerSuccessGlow();
    } catch (e) {
      encryptedForensicDate = "";
      showSaveToastMessage("Encryption failed: " + e, "error");
    } finally {
      cryptoBusy = false;
    }
  }

  function triggerSuccessGlow() {
    const preview = document.querySelector('.preview-area');
    if (preview) {
      preview.classList.add('success-glow');
      setTimeout(() => preview.classList.remove('success-glow'), 2000);
    }
  }

  async function decryptEncryptedQrPayload() {
    if (!decryptPayload.trim()) {
      showSaveToastMessage("Paste or scan an encrypted QR payload first.", "error");
      return;
    }
    if (!decryptPassphrase) {
      showSaveToastMessage("Enter the passphrase before decrypting.", "error");
      return;
    }

    cryptoBusy = true;
    decryptedPlaintext = "";
    decryptedForensicDate = "";
    try {
      decryptedPlaintext = await invoke<string>("decrypt_qr_payload", {
        encryptedPayload: decryptPayload.trim(),
        passphrase: decryptPassphrase
      });
      decryptedForensicDate = getPayloadForensicDate(decryptPayload);
      showSaveToastMessage(
        decryptedForensicDate
          ? `Payload verified. Forensic date: ${decryptedForensicDate}.`
          : "Decrypted payload locally. No forensic date found.",
        "success"
      );
      
      triggerSuccessGlow();
    } catch (e) {
      showSaveToastMessage("Decryption failed: " + e, "error");
    } finally {
      cryptoBusy = false;
    }
  }

  function getGeneratedLabel() {
    if ((dataType === "URL" || dataType === "Text") && isEncryptedQrPayload(qrData)) return "Encrypted QR Payload";
    if (dataType === "DogTag" && petName.trim()) return `${petName.trim()} Dog Tag`;
    if (dataType === "WiFi" && wifiSsid.trim()) return `${wifiSsid.trim()} WiFi`;
    if (dataType === "vCard" && (vCardFirst.trim() || vCardLast.trim())) return `${vCardFirst.trim()} ${vCardLast.trim()}`.trim();
    if (dataType === "Email" && emailTo.trim()) return `Email for ${emailTo.trim()}`;
    if (dataType === "SMS" && smsPhone.trim()) return `SMS for ${smsPhone.trim()}`;
    if (dataType === "Phone" && phoneNum.trim()) return `Phone ${phoneNum.trim()}`;
    if (dataType === "Geo" && geoLat.trim() && geoLng.trim()) return `Location ${geoLat.trim()}, ${geoLng.trim()}`;
    if (dataType === "WhatsApp" && waPhone.trim()) return `WhatsApp ${waPhone.trim()}`;
    if (dataType === "Crypto" && walletName.trim()) return walletName.trim();
    if (dataType === "Crypto" && cryptoType.trim()) return `${toDisplayCase(cryptoType.trim())} Wallet`;
    if (dataType === "Event" && eventTitle.trim()) return eventTitle.trim();
    if (dataType === "Social" && socialUser.trim()) return `${socialPlatform} ${socialUser.trim()}`;
    if (dataType === "LinkedIn" && linkedinUser.trim()) return `LinkedIn ${linkedinUser.trim()}`;
    if (dataType === "YouTube" && ytHandle.trim()) return `YouTube ${ytHandle.trim()}`;
    if (dataType === "TikTok" && tiktokUser.trim()) return `TikTok ${tiktokUser.trim()}`;
    if (dataType === "Zoom" && zoomMeetingId.trim()) return `Zoom ${zoomMeetingId.trim()}`;
    if (dataType === "URL" && qrData.trim()) return qrData.trim();
    if (dataType === "Text" && qrData.trim()) {
      const clean = qrData.trim();
      return clean.length > 20 ? clean.slice(0, 17) + "..." : clean;
    }
    return `${dataType} QR Code`;
  }

  function getPrintPayloadPreview() {
    const compact = generatedPayload.replace(/\s+/g, " ").trim();
    return compact.length > 140 ? `${compact.slice(0, 137)}...` : compact;
  }

  async function printCode() {
    if (!qrImagePng) return;
    const printHeading = printTitle.trim() || generatedLabel;

    if (isNativeMobileDevice()) {
      try {
        const msg = await invoke<string>("print_current_image", { b64: qrImagePng, title: printHeading });
        showSaveToastMessage(msg, "info");
      } catch (e) {
        showSaveToastMessage("Could not open print dialog: " + e, "error");
      }
      return;
    }

    if (typeof window === "undefined" || typeof window.print !== "function") {
      showSaveToastMessage("Printing is not available on this device.", "error");
      return;
    }

    await tick();
    window.print();
  }

  function encodeTextToBase64(value: string): string {
    const bytes = new TextEncoder().encode(value);
    let binary = "";
    const chunkSize = 0x8000;

    for (let i = 0; i < bytes.length; i += chunkSize) {
      const chunk = bytes.subarray(i, i + chunkSize);
      binary += String.fromCharCode(...chunk);
    }

    return btoa(binary);
  }
  // --- SAVE TO NATIVE GALLERY ---
  async function saveImage() {
    if (!qrImagePng) return;
    const finalData = buildFinalQrData();
    const isSvg = saveFormat === "svg";
    const b64Data = isSvg ? "" : (saveFormat === "jpg" ? qrImageJpg : qrImagePng);
    mobileSaveMessage = "";
    showMobileSaveActions = false;
    
    try {
      const isMobile = isNativeMobileDevice();

      if (isMobile) {
        if (isSvg) {
          const svg = await invoke<string>("generate_ultra_qr_svg", {
            options: buildQrRenderOptions(finalData)
          });
          const svgB64 = encodeTextToBase64(svg);
          const result = await invoke<{ message: string }>("save_to_device", { b64: svgB64, format: "svg" });
          showSaveToastMessage(result.message, "success");
          showMobileSaveActions = true;
          rememberRecentSave("Saved SVG to device");
        } else {
          const result = await invoke<{ message: string }>("save_to_device", { b64: b64Data, format: saveFormat });
          showSaveToastMessage(result.message, "success");
          showMobileSaveActions = true;
          rememberRecentSave(`Saved ${saveFormat.toUpperCase()} to Gallery`);
        }
      } else {
        // Prompt user to choose where to save on desktop
        const filePath = await save({
          filters: [{
            name: 'Image',
            extensions: [saveFormat]
          }],
          defaultPath: `QR_Studio_${Date.now()}.${saveFormat}`
        });

        if (filePath) {
          const msg = isSvg
            ? await invoke("save_svg_to_path", { options: buildQrRenderOptions(finalData), path: filePath })
            : await invoke("save_to_path", { b64: b64Data, path: filePath });
          showSaveToastMessage(String(msg), "success");
          rememberRecentSave(`Saved ${saveFormat.toUpperCase()} locally`);
        }
      }
    } catch (e) {
      showSaveToastMessage("Failed to save: " + e, "error");
    }
  }

  async function openSavedImage() {
    try {
      const msg = await invoke<string>("open_last_saved_image");
      showSaveToastMessage(msg, "info");
    } catch (e) {
      showSaveToastMessage("Could not open saved image: " + e, "error");
    }
  }

  async function shareSavedImage() {
    try {
      const msg = await invoke<string>("share_last_saved_image");
      showSaveToastMessage(msg, "info");
    } catch (e) {
      showSaveToastMessage("Could not share saved image: " + e, "error");
    }
  }

  // --- NATIVE SCANNER FUNCTIONS ---
  async function startScan() {
    scannedResult = ""; 
    scannedFormat = "";
    scannedKind = "";
    closeScanDecryptModal();
    try {
      try {
        await requestPermissions();
      } catch (permErr) {
        showSaveToastMessage("Camera access denied. Please grant camera permission on your mobile device to use the scanner.", "error");
        return;
      }

      isScanning = true;
      const result = await scan({ windowed: true, cameraDirection: "back", formats: scannerFormats });
      
      if (result && result.content) {
         const payload = result.content.trim();
         scannedFormat = toDisplayCase(result.format);
         scannedResult = payload;
         scannedKind = classifyScan(payload, result.format);
         if (isEncryptedQrPayload(payload)) {
           handleEncryptedQrScan(payload);
         }
      }
    } catch (e) {
      if (e !== "Canceled" && e !== "cancel") {
         showSaveToastMessage("Scanner could not start: " + e, "error");
      }
    } finally {
      isScanning = false;
    }
  }

  async function cancelScanner() {
    try { await cancel(); } catch (e) { console.error(e); }
    isScanning = false;
  }

  async function openLink(url: string) {
      let finalUrl = url.trim();
      if (!/^[a-zA-Z0-9\-]+:/i.test(finalUrl)) {
          finalUrl = 'https://' + finalUrl;
      }
      
      try {
          await invoke("open_external_link", { url: finalUrl });
      } catch (err) {
          try {
              if (navigator.share) {
                  await navigator.share({ url: finalUrl });
              } else {
                  showSaveToastMessage("Could not open link: " + err, "error");
              }
          } catch (shareErr) {}
      }
  }

  async function copyText(text: string) {
      try {
          await navigator.clipboard.writeText(text);
          showSaveToastMessage("Copied to clipboard.", "success");
      } catch (err) {
          const textArea = document.createElement("textarea");
          textArea.value = text;
          textArea.setAttribute("readonly", "");
          textArea.style.position = "fixed";
          textArea.style.left = "-9999px";
          document.body.appendChild(textArea);
          textArea.select();
          try {
            const copied = document.execCommand("copy");
            showSaveToastMessage(copied ? "Copied to clipboard." : "Clipboard failed. Please copy manually.", copied ? "success" : "error");
          } catch {
            showSaveToastMessage("Clipboard failed. Please copy manually.", "error");
          } finally {
            document.body.removeChild(textArea);
          }
      }
  }

  function getPassphraseStrength(passphrase: string) {
    let score = 0;
    if (passphrase.length >= 10) score++;
    if (passphrase.length >= 16) score++;
    if (/[a-z]/.test(passphrase) && /[A-Z]/.test(passphrase)) score++;
    if (/\d/.test(passphrase)) score++;
    if (/[^A-Za-z0-9]/.test(passphrase)) score++;

    if (!passphrase) return { label: "Not set", className: "empty", percent: 0 };
    if (score <= 2) return { label: "Weak", className: "weak", percent: 34 };
    if (score <= 4) return { label: "Good", className: "good", percent: 68 };
    return { label: "Strong", className: "strong", percent: 100 };
  }

  async function copySvg() {
    if (!qrImagePng) return;
    try {
      const svg = await invoke<string>("generate_ultra_qr_svg", {
        options: buildQrRenderOptions(generatedPayload || buildFinalQrData())
      });
      await navigator.clipboard.writeText(svg);
      showSaveToastMessage("SVG copied to clipboard.", "success");
    } catch (e) {
      showSaveToastMessage("Could not copy SVG: " + e, "error");
    }
  }

  async function quickSave(format: string) {
    saveFormat = format;
    await saveImage();
  }

  // --- VALIDATION ---
  function validateInputs() {
    if (dataType === "URL" && !qrData.trim()) return "Please enter a valid URL.";
    if (dataType === "Text" && !qrData.trim()) return "Please enter the text content.";
    if (dataType === "WiFi" && !wifiSsid.trim()) return "Please enter the WiFi Network Name (SSID).";
    if (dataType === "vCard" && (!vCardFirst.trim() || !vCardPhone.trim())) return "Please enter at least a First Name and Phone Number for the contact.";
    if (dataType === "Email" && !emailTo.trim()) return "Please enter a destination Email Address.";
    if (dataType === "SMS" && !smsPhone.trim()) return "Please enter a destination Phone Number.";
    if (dataType === "Phone" && !phoneNum.trim()) return "Please enter a Phone Number.";
    if (dataType === "Crypto" && !cryptoAddr.trim()) return "Please enter a wallet address.";
    
    if (dataType === "Geo") {
      if (!geoLat.trim() || !geoLng.trim()) return "Please enter both Latitude and Longitude.";
      if (isNaN(parseFloat(geoLat)) || isNaN(parseFloat(geoLng))) return "Latitude and Longitude must be valid numbers.";
    }
    
    if (dataType === "DogTag") {
      if (!petName.trim() || !ownerPhone.trim()) return "Please enter at least the Pet's Name and an Owner Phone Number.";
      if (microchipNum.trim() && microchipNum.trim().length !== 15) return "Microchip number must be exactly 15 digits.";
    }
    
    return ""; 
  }

  // --- GENERATION FLOW ---
  function handleGenerateClick() {
    const errorMsg = validateInputs();
    if (errorMsg) {
      showSaveToastMessage(errorMsg, "error");
      return;
    }

    if (dataType === "DogTag" && !microchipNum.trim()) {
       showDogTagWarning = true;
       return;
    }

    runGeneration();
  }

  // --- CONSOLIDATED CANVAS HELPERS ---
  const drawShapePath = (c: CanvasRenderingContext2D, s: string, cx: number, cy: number, sz: number) => {
    c.beginPath();
    if (s === "circle") {
      c.arc(cx, cy, sz / 2, 0, Math.PI * 2);
    } else if (s === "rounded") {
      const r = sz * 0.2;
      const left = cx - sz / 2, top = cy - sz / 2, right = cx + sz / 2, bottom = cy + sz / 2;
      c.moveTo(cx, top);
      c.arcTo(right, top, right, bottom, r);
      c.arcTo(right, bottom, left, bottom, r);
      c.arcTo(left, bottom, left, top, r);
      c.arcTo(left, top, right, top, r);
      c.closePath();
    } else if (s === "diamond") {
      c.moveTo(cx, cy - sz / 2);
      c.lineTo(cx + sz / 2, cy);
      c.lineTo(cx, cy + sz / 2);
      c.lineTo(cx - sz / 2, cy);
      c.closePath();
    } else if (s === "octagon") {
      const side = sz * 0.28;
      const left = cx - sz / 2, top = cy - sz / 2, right = cx + sz / 2, bottom = cy + sz / 2;
      c.moveTo(cx - side, top);
      c.lineTo(cx + side, top);
      c.lineTo(right, top + side);
      c.lineTo(right, bottom - side);
      c.lineTo(cx + side, bottom);
      c.lineTo(cx - side, bottom);
      c.lineTo(left, bottom - side);
      c.lineTo(left, top + side);
      c.closePath();
    } else {
      c.rect(cx - sz / 2, cy - sz / 2, sz, sz);
    }
  };

  const drawShapePathRect = (
    c: CanvasRenderingContext2D,
    s: string,
    cx: number,
    cy: number,
    width: number,
    height: number
  ) => {
    c.beginPath();
    const halfW = width / 2;
    const halfH = height / 2;
    if (s === "circle") {
      c.ellipse(cx, cy, halfW, halfH, 0, 0, Math.PI * 2);
    } else if (s === "rounded") {
      const r = Math.min(width, height) * 0.2;
      const left = cx - halfW, top = cy - halfH, right = cx + halfW, bottom = cy + halfH;
      c.moveTo(cx, top);
      c.arcTo(right, top, right, bottom, r);
      c.arcTo(right, bottom, left, bottom, r);
      c.arcTo(left, bottom, left, top, r);
      c.arcTo(left, top, right, top, r);
      c.closePath();
    } else if (s === "diamond") {
      c.moveTo(cx, cy - halfH);
      c.lineTo(cx + halfW, cy);
      c.lineTo(cx, cy + halfH);
      c.lineTo(cx - halfW, cy);
      c.closePath();
    } else if (s === "octagon") {
      const sideX = width * 0.28;
      const sideY = height * 0.28;
      const left = cx - halfW, top = cy - halfH, right = cx + halfW, bottom = cy + halfH;
      c.moveTo(cx - sideX, top);
      c.lineTo(cx + sideX, top);
      c.lineTo(right, cy - sideY);
      c.lineTo(right, cy + sideY);
      c.lineTo(cx + sideX, bottom);
      c.lineTo(cx - sideX, bottom);
      c.lineTo(left, cy + sideY);
      c.lineTo(left, cy - sideY);
      c.closePath();
    } else {
      c.rect(cx - halfW, cy - halfH, width, height);
    }
  };

  const drawRingDecoration = (
    c: CanvasRenderingContext2D,
    style: string,
    shape: string,
    cx: number,
    cy: number,
    size: number,
    color: string,
    palette?: { c1: string; c2: string; c3?: string; c4?: string },
    useGradientColor = false
  ) => {
    if (style === "none") return;

    c.save();
    c.strokeStyle = color;
    c.fillStyle = color;

    if (style === "gradient" || useGradientColor) {
      const grad = c.createLinearGradient(cx - size / 2, cy - size / 2, cx + size / 2, cy + size / 2);
      const gradientPalette = palette ?? getGradientPalette(color1, color2, color3, color4, useFourthStop);
      applyGradientStops(
        grad,
        gradientPalette.c1,
        gradientPalette.c2,
        gradientPalette.c3,
        gradientPalette.c4,
        Boolean(gradientPalette.c4)
      );
      c.strokeStyle = grad;
      c.fillStyle = grad;
    }

    const baseLine = Math.max(4, size * 0.034);
    const half = size / 2;

    if (style === "double") {
      c.lineWidth = Math.max(2, size * 0.012);
      const offset = size * 0.034;
      drawShapePath(c, shape, cx, cy, size + offset); c.stroke();
      drawShapePath(c, shape, cx, cy, size - offset); c.stroke();
    } else if (style === "dotted") {
      c.lineWidth = baseLine;
      c.setLineDash([2, Math.max(10, size * 0.064)]);
      c.lineCap = "round";
      drawShapePath(c, shape, cx, cy, size); c.stroke();
    } else if (style === "dashed") {
      c.lineWidth = baseLine;
      c.setLineDash([Math.max(12, size * 0.072), Math.max(10, size * 0.043)]);
      drawShapePath(c, shape, cx, cy, size); c.stroke();
    } else if (style === "rounded") {
      c.lineWidth = baseLine;
      c.setLineDash([Math.max(10, size * 0.043), Math.max(8, size * 0.028)]);
      c.lineCap = "round";
      drawShapePath(c, shape, cx, cy, size); c.stroke();
    } else if (style === "diamond") {
      const count = Math.max(16, Math.round(size / 16));
      const diamondSize = Math.max(6, size * 0.034);
      for (let i = 0; i < count; i++) {
        const angle = (i / count) * Math.PI * 2;
        let x = Math.cos(angle) * half;
        let y = Math.sin(angle) * half;

        if (shape === "square" || shape === "diamond") {
          if (shape === "diamond") {
            const angleRot = angle + Math.PI / 4;
            let rx = Math.cos(angleRot);
            let ry = Math.sin(angleRot);
            const scale = (half / Math.sqrt(2)) / Math.max(Math.abs(rx), Math.abs(ry));
            const tx = rx * scale;
            const ty = ry * scale;
            x = (tx - ty) / Math.sqrt(2);
            y = (tx + ty) / Math.sqrt(2);
          } else {
            const scale = half / Math.max(Math.abs(Math.cos(angle)), Math.abs(Math.sin(angle)));
            x = Math.cos(angle) * scale;
            y = Math.sin(angle) * scale;
          }
        }

        c.save();
        c.translate(cx + x, cy + y);
        c.rotate(angle + Math.PI / 4);
        c.fillRect(-diamondSize / 2, -diamondSize / 2, diamondSize, diamondSize);
        c.restore();
      }
    } else if (style === "neon") {
      c.lineWidth = Math.max(4, size * 0.028);
      c.shadowColor = color;
      c.shadowBlur = Math.max(10, size * 0.038);
      drawShapePath(c, shape, cx, cy, size); c.stroke();
      c.stroke();
      c.strokeStyle = "#FFFFFF";
      c.lineWidth = Math.max(2, size * 0.006);
      c.shadowBlur = 0;
      drawShapePath(c, shape, cx, cy, size); c.stroke();
    } else {
      c.lineWidth = baseLine;
      drawShapePath(c, shape, cx, cy, size); c.stroke();
    }

    c.restore();
  };

  const drawRingDecorationRect = (
    c: CanvasRenderingContext2D,
    style: string,
    shape: string,
    cx: number,
    cy: number,
    width: number,
    height: number,
    color: string,
    palette?: { c1: string; c2: string; c3?: string; c4?: string },
    useGradientColor = false
  ) => {
    if (style === "none") return;
    c.save();
    c.lineJoin = "round";
    c.lineCap = "round";
    c.strokeStyle = color;
    c.fillStyle = color;

    if (style === "gradient" || useGradientColor) {
      const grad = c.createLinearGradient(cx - width / 2, cy - height / 2, cx + width / 2, cy + height / 2);
      const gradientPalette = palette ?? getGradientPalette(color1, color2, color3, color4, useFourthStop);
      applyGradientStops(
        grad,
        gradientPalette.c1,
        gradientPalette.c2,
        gradientPalette.c3,
        gradientPalette.c4,
        Boolean(gradientPalette.c4)
      );
      c.strokeStyle = grad;
      c.fillStyle = grad;
    }

    const baseLine = Math.max(4, Math.min(width, height) * 0.04);

    if (style === "dashed") {
      c.setLineDash([baseLine * 2, baseLine * 2]);
      c.lineWidth = baseLine;
      drawShapePathRect(c, shape, cx, cy, width, height); c.stroke();
    } else if (style === "dotted") {
      c.lineWidth = baseLine;
      c.setLineDash([2, 6]);
      drawShapePathRect(c, shape, cx, cy, width, height); c.stroke();
    } else if (style === "double") {
      c.lineWidth = baseLine * 0.4;
      drawShapePathRect(c, shape, cx, cy, width, height); c.stroke();
      drawShapePathRect(c, shape, cx, cy, width - baseLine * 1.5, height - baseLine * 1.5); c.stroke();
    } else if (style === "neon") {
      c.shadowBlur = baseLine * 2;
      c.shadowColor = color;
      c.lineWidth = baseLine;
      drawShapePathRect(c, shape, cx, cy, width, height); c.stroke();
      c.strokeStyle = "#FFFFFF";
      c.lineWidth = Math.max(2, Math.min(width, height) * 0.006);
      c.shadowBlur = 0;
      drawShapePathRect(c, shape, cx, cy, width, height); c.stroke();
    } else {
      c.lineWidth = baseLine;
      drawShapePathRect(c, shape, cx, cy, width, height); c.stroke();
    }

    c.restore();
  };

  const drawGlobalCenterOverlay = (c: CanvasRenderingContext2D, centerX: number, centerY: number, renderedQrSize: number) => {
    if (!logoBase64 || centerOverlayMode === "none") return;
    if (centerOverlayMode === "match" && !enableFrame) return;

    const overlayStyle = centerOverlayMode === "match" && enableFrame ? ringStyle : centerOverlayStyle;
    const overlayColor = centerOverlayMode === "match" && enableFrame ? ringColor : centerOverlayColor;
    const overlayPalette = centerOverlayMode === "match" && enableFrame ? getRingPalette() : getCenterOverlayPalette();
    const overlayUsesGradient = centerOverlayMode === "match" && enableFrame
      ? (ringStyle !== "solid" && ringStyle !== "none" && ringColorMode === "gradient")
      : (centerOverlayStyle !== "solid" && centerOverlayStyle !== "none" && centerOverlayColorMode === "gradient");

    if (overlayStyle === "none") return;
    const dimensions = getCenterOverlayDimensions(renderedQrSize);
    drawRingDecorationRect(
      c,
      overlayStyle,
      bgShape,
      centerX,
      centerY,
      dimensions.width,
      dimensions.height,
      overlayColor,
      overlayPalette,
      overlayUsesGradient
    );
  };

  const getPathPoint = (shape: string, progress: number, radius: number, cx = 400, cy = 400): { x: number; y: number; angle: number } => {
    let x = cx, y = cy, angle = 0;

    if (shape === "circle") {
      const a = progress * Math.PI * 2 - Math.PI / 2;
      x = cx + Math.cos(a) * radius;
      y = cy + Math.sin(a) * radius;
      angle = a + Math.PI / 2;
    } else if (shape === "square") {
      const side = radius * 2;
      const p = (progress + 0.125) % 1; // Offset to start at top center
      if (p < 0.25) { // Top
        const f = p / 0.25;
        x = cx - radius + f * side;
        y = cy - radius;
        angle = 0;
      } else if (p < 0.5) { // Right
        const f = (p - 0.25) / 0.25;
        x = cx + radius;
        y = cy - radius + f * side;
        angle = Math.PI / 2;
      } else if (p < 0.75) { // Bottom
        const f = (p - 0.5) / 0.25;
        x = cx + radius - f * side;
        y = cy + radius;
        angle = Math.PI;
      } else { // Left
        const f = (p - 0.75) / 0.25;
        x = cx - radius;
        y = cy + radius - f * side;
        angle = -Math.PI / 2;
      }
    } else if (shape === "diamond") {
      const p = (progress + 0.125) % 1;
      const s = radius * Math.sqrt(2);
      if (p < 0.25) { // Top-Right
        const f = p / 0.25;
        x = cx + f * radius;
        y = cy - radius + f * radius;
        angle = Math.PI / 4;
      } else if (p < 0.5) { // Bottom-Right
        const f = (p - 0.25) / 0.25;
        x = cx + radius - f * radius;
        y = cy + f * radius;
        angle = 3 * Math.PI / 4;
      } else if (p < 0.75) { // Bottom-Left
        const f = (p - 0.5) / 0.25;
        x = cx - f * radius;
        y = cy + radius - f * radius;
        angle = -3 * Math.PI / 4;
      } else { // Top-Left
        const f = (p - 0.75) / 0.25;
        x = cx - radius + f * radius;
        y = cy - f * radius;
        angle = -Math.PI / 4;
      }
    } else if (shape === "octagon") {
      const p = progress % 1;
      const side = radius * 0.28 * 2;
      const corner = (radius - radius * 0.28);
      // Simplified octagon path: 8 segments
      const seg = Math.floor(p * 8);
      const f = (p * 8) % 1;
      const pts = [
        {x: cx-radius*0.28, y: cy-radius, a: 0},
        {x: cx+radius*0.28, y: cy-radius, a: Math.PI/4},
        {x: cx+radius, y: cy-radius*0.28, a: Math.PI/2},
        {x: cx+radius, y: cy+radius*0.28, a: 3*Math.PI/4},
        {x: cx+radius*0.28, y: cy+radius, a: Math.PI},
        {x: cx-radius*0.28, y: cy+radius, a: -3*Math.PI/4},
        {x: cx-radius, y: cy+radius*0.28, a: -Math.PI/2},
        {x: cx-radius, y: cy-radius*0.28, a: -Math.PI/4}
      ];
      const p1 = pts[seg];
      const p2 = pts[(seg+1)%8];
      x = p1.x + (p2.x - p1.x) * f;
      y = p1.y + (p2.y - p1.y) * f;
      angle = p1.a;
    } else if (shape === "rounded") {
      const r = radius * 0.2;
      const side = (radius - r) * 2;
      const arcLen = Math.PI * r / 2;
      const totalLen = 4 * side + 4 * arcLen;
      let d = progress * totalLen;

      // Start from top center
      d = (d + side / 2 + totalLen) % totalLen;

      if (d < side) { // Top side
        x = cx - (radius-r) + d; y = cy - radius; angle = 0;
      } else if (d < side + arcLen) { // Top-right arc
        const a = (d - side) / arcLen * (Math.PI/2) - Math.PI/2;
        x = cx + (radius-r) + Math.cos(a) * r;
        y = cy - (radius-r) + Math.sin(a) * r;
        angle = a + Math.PI/2;
      } else if (d < 2*side + arcLen) { // Right side
        x = cx + radius; y = cy - (radius-r) + (d - side - arcLen); angle = Math.PI/2;
      } else if (d < 2*side + 2*arcLen) { // Bottom-right arc
        const a = (d - 2*side - arcLen) / arcLen * (Math.PI/2);
        x = cx + (radius-r) + Math.cos(a) * r;
        y = cy + (radius-r) + Math.sin(a) * r;
        angle = a + Math.PI/2;
      } else if (d < 3*side + 2*arcLen) { // Bottom side
        x = cx + (radius-r) - (d - 2*side - 2*arcLen); y = cy + radius; angle = Math.PI;
      } else if (d < 3*side + 3*arcLen) { // Bottom-left arc
        const a = (d - 3*side - 2*arcLen) / arcLen * (Math.PI/2) + Math.PI/2;
        x = cx - (radius-r) + Math.cos(a) * r;
        y = cy + (radius-r) + Math.sin(a) * r;
        angle = a + Math.PI/2;
      } else if (d < 4*side + 3*arcLen) { // Left side
        x = cx - radius; y = cy + (radius-r) - (d - 3*side - 3*arcLen); angle = -Math.PI/2;
      } else { // Top-left arc
        const a = (d - 4*side - 3*arcLen) / arcLen * (Math.PI/2) + Math.PI;
        x = cx - (radius-r) + Math.cos(a) * r;
        y = cy - (radius-r) + Math.sin(a) * r;
        angle = a + Math.PI/2;
      }
    } else {
      // Default fallback (circle)
      const a = progress * Math.PI * 2 - Math.PI / 2;
      x = cx + Math.cos(a) * radius;
      y = cy + Math.sin(a) * radius;
      angle = a + Math.PI / 2;
    }
    return { x, y, angle };
  };

  const getFrameRingClearance = (style: string, size: number, textSize: number, isTopText = false) => {
    const baseLine = Math.max(4, size * 0.034);
    let ringInset = baseLine / 2;

    if (style === "double") {
      ringInset = size * 0.034 + Math.max(2, size * 0.012) / 2;
    } else if (style === "diamond") {
      ringInset = Math.max(6, size * 0.034) / 2 + 8;
    } else if (style === "neon") {
      ringInset = Math.max(4, size * 0.028) / 2 + 10;
    }

    const topTextBreathingRoom = isTopText ? 24 : 0;
    return ringInset + textSize * 0.65 + 10 + topTextBreathingRoom;
  };

  const drawFrameText = (ctx: CanvasRenderingContext2D, text: string, isTop: boolean, cx = 400, cy = 400) => {
    if (!text) return;

    const currentSize = isTop ? frameTextTopSize : frameTextSize;
    const currentColor = isTop ? frameTextTopColor : frameTextColor;
    const currentMatchStyle = isTop ? matchTextTopStyle : matchTextStyle;
    const currentTransparentBg = isTop ? transparentTextTopBg : transparentTextBg;
    const currentMode = isTop ? frameTextTopMode : frameTextMode;
    const currentRadius = isTop ? frameTextTopRadius : frameTextRadius;
    const currentSpacing = isTop ? frameTextTopSpacing : frameTextSpacing;

    const applyTextColor = (textWidth: number, currentMatchStyle: boolean, currentColor: string, isCurved = false) => {
      if (currentMatchStyle) {
        if (ringStyle === "gradient" || (ringStyle !== "solid" && ringStyle !== "none" && ringColorMode === "gradient")) {
          const gradX0 = isCurved ? -textWidth / 2 : cx - textWidth / 2;
          const gradX1 = isCurved ? textWidth / 2 : cx + textWidth / 2;
          const grad = ctx.createLinearGradient(gradX0, 0, gradX1, 0);
          const ringPalette = getRingPalette();
          applyGradientStops(grad, ringPalette.c1, ringPalette.c2, ringPalette.c3, ringPalette.c4, Boolean(ringPalette.c4));
          ctx.fillStyle = grad;
        } else if (ringStyle === "neon") {
          ctx.fillStyle = "#FFFFFF";
          ctx.shadowColor = ringColor;
          ctx.shadowBlur = 10;
        } else if (ringStyle === "diamond") {
          ctx.fillStyle = ringColor;
        } else {
          ctx.fillStyle = currentColor;
        }
      } else {
        ctx.fillStyle = currentColor;
      }
    };

    ctx.save();
    ctx.font = `bold ${currentSize}px 'Segoe UI', Arial, sans-serif`;
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";

    if (currentMode === "curved") {
      const safeRadius = 700 / 2 - getFrameRingClearance(ringStyle, 700, currentSize, isTop);
      const curvedRadius = Math.max(200, Math.min(currentRadius, safeRadius));
      const chars = text.toUpperCase().split("");
      const charWidths = chars.map(c => ctx.measureText(c).width + currentSpacing);
      const totalTextWidth = charWidths.reduce((a, b) => a + b, 0);

      let perimeter = 2 * Math.PI * curvedRadius;
      if (bgShape === "square") perimeter = 8 * curvedRadius;
      else if (bgShape === "diamond") perimeter = 4 * curvedRadius * Math.sqrt(2);
      else if (bgShape === "octagon") perimeter = 8 * curvedRadius * 0.828;
      else if (bgShape === "rounded") {
          const side = (curvedRadius - curvedRadius * 0.2) * 2;
          const arc = Math.PI * (curvedRadius * 0.2) / 2;
          perimeter = 4 * side + 4 * arc;
      }

      const widthToProgress = 1 / perimeter;
      const direction = isTop ? 1 : -1;
      const startProgress = isTop
        ? -(totalTextWidth / 2) * widthToProgress
        : 0.5 + (totalTextWidth / 2) * widthToProgress;

      let currentProgress = startProgress;
      chars.forEach((char, i) => {
        const charProgress = currentProgress + direction * (charWidths[i] / 2) * widthToProgress;
        const { x, y, angle } = getPathPoint(bgShape, charProgress, curvedRadius, cx, cy);

        ctx.save();
        ctx.translate(x, y);
        ctx.rotate(isTop ? angle : angle + Math.PI);

        applyTextColor(totalTextWidth, currentMatchStyle, currentColor, true);
        ctx.fillText(char, 0, 0);
        ctx.restore();

        currentProgress += direction * charWidths[i] * widthToProgress;
      });
    } else {
      ctx.setTransform(1, 0, 0, 1, 0, 0);
      const textWidth = ctx.measureText(text.toUpperCase()).width;

      let badgeY = isTop ? 112 : 700;
      if (bgShape === "diamond") badgeY = isTop ? 166 : 640;
      else if (bgShape === "octagon") badgeY = isTop ? 140 : 670;
      else if (bgShape === "rounded") badgeY = isTop ? 122 : 690;
      else if (bgShape === "circle") badgeY = isTop ? 112 : 700;

      const badgeWidth = textWidth + 80;
      const badgeHeight = 70;

      if (!currentTransparentBg) {
        ctx.fillStyle = bgColor;
        if (currentMatchStyle && ringStyle === "neon") {
          ctx.shadowColor = ringColor;
          ctx.shadowBlur = 20;
        }

        const bx = cx - badgeWidth / 2;
        const by = badgeY - badgeHeight / 2;
        if (currentMatchStyle && (ringStyle === "rounded" || bgShape === "rounded")) {
          const r = 15;
          ctx.beginPath();
          ctx.moveTo(bx + r, by);
          ctx.lineTo(bx + badgeWidth - r, by);
          ctx.quadraticCurveTo(bx + badgeWidth, by, bx + badgeWidth, by + r);
          ctx.lineTo(bx + badgeWidth, by + badgeHeight - r);
          ctx.quadraticCurveTo(bx + badgeWidth, by + badgeHeight, bx + badgeWidth - r, by + badgeHeight);
          ctx.lineTo(bx + r, by + badgeHeight);
          ctx.quadraticCurveTo(bx, by + badgeHeight, bx, by + badgeHeight - r);
          ctx.lineTo(bx, by + r);
          ctx.quadraticCurveTo(bx, by, bx + r, by);
          ctx.closePath();
          ctx.fill();
        } else {
          ctx.fillRect(bx, by, badgeWidth, badgeHeight);
        }

        if (currentMatchStyle) {
          ctx.shadowBlur = 0;
          ctx.strokeStyle = ringColor;
          ctx.lineWidth = 3;
          if (ringStyle === "dotted") ctx.setLineDash([2, 6]);
          else if (ringStyle === "dashed") ctx.setLineDash([12, 6]);
          else if (ringStyle === "double") ctx.lineWidth = 1;

          if (ringStyle !== "none" && ringStyle !== "neon" && ringStyle !== "gradient") {
            if (ringStyle === "rounded" || bgShape === "rounded") {
              ctx.stroke();
              if (ringStyle === "double") {
                ctx.save();
                ctx.translate(2, 2); ctx.scale((badgeWidth-4)/badgeWidth, (badgeHeight-4)/badgeHeight);
                ctx.stroke();
                ctx.restore();
              }
            } else {
              ctx.strokeRect(bx, by, badgeWidth, badgeHeight);
              if (ringStyle === "double") {
                ctx.strokeRect(bx + 5, by + 5, badgeWidth - 10, badgeHeight - 10);
              }
            }
          }
          ctx.setLineDash([]);
        }
      }

      applyTextColor(textWidth, currentMatchStyle, currentColor);
      ctx.fillText(text.toUpperCase(), cx, badgeY);
    }
    ctx.restore();
  };


  async function runGeneration() {
    showDogTagWarning = false;
    loading = true;
    const finalData = buildFinalQrData();

    try {
      const rustImageB64 = await invoke<string>("generate_ultra_qr", {
        options: buildQrRenderOptions(finalData)
      });

      const img = new Image();
      img.onload = () => {
        try {
          const canvas = document.createElement("canvas");
          const ctx = canvas.getContext("2d");
          if (!ctx) {
            throw new Error("Preview canvas context could not be created.");
          }

          if (enableFrame) {
            canvas.width = 800;
            canvas.height = 800;

            ctx.save();
            drawShapePath(ctx, bgShape, 400, 400, 800);
            ctx.clip();

            if (!transparentFrameBg) {
              ctx.fillStyle = bgColor;
              ctx.fillRect(0, 0, 800, 800);
            }

            drawRingDecoration(
              ctx,
              ringStyle,
              bgShape,
              400,
              400,
              700,
              ringColor,
              getRingPalette(),
              ringStyle !== "solid" && ringStyle !== "none" && ringColorMode === "gradient"
            );

            ctx.save();
            drawShapePath(ctx, bgShape, 400, 400, 660);
            ctx.clip();

            let qrSize = 440;
            if (bgShape === "diamond") qrSize = 340;
            else if (bgShape === "octagon") qrSize = 400;
            else if (bgShape === "rounded") qrSize = 460;
            else if (bgShape === "square") qrSize = 480;
            const qrOffset = (800 - qrSize) / 2;

            ctx.drawImage(img, qrOffset, qrOffset, qrSize, qrSize);
            drawGlobalCenterOverlay(ctx, 400, 400, qrSize);
            ctx.restore();

            drawFrameText(ctx, frameTextTop, true);
            drawFrameText(ctx, frameText, false);
            ctx.restore();
          } else {
            canvas.width = 600;
            canvas.height = 600;
            if (bgShape === "circle") {
              ctx.save();
              ctx.beginPath();
              ctx.arc(300, 300, 300, 0, Math.PI * 2);
              ctx.clip();
              ctx.fillStyle = bgColor;
              ctx.fillRect(0, 0, 600, 600);
              ctx.drawImage(img, 100, 100, 400, 400);
              ctx.restore();
            } else if (bgShape === "rounded") {
              const r = 120;
              ctx.save();
              ctx.beginPath();
              ctx.moveTo(300, 0);
              ctx.arcTo(600, 0, 600, 600, r);
              ctx.arcTo(600, 600, 0, 600, r);
              ctx.arcTo(0, 600, 0, 0, r);
              ctx.arcTo(0, 0, 600, 0, r);
              ctx.closePath();
              ctx.clip();
              ctx.fillStyle = bgColor;
              ctx.fillRect(0, 0, 600, 600);
              ctx.drawImage(img, 10, 10, 580, 580);
              ctx.restore();
            } else if (bgShape === "diamond") {
              ctx.save();
              ctx.beginPath();
              ctx.moveTo(300, 0);
              ctx.lineTo(600, 300);
              ctx.lineTo(300, 600);
              ctx.lineTo(0, 300);
              ctx.closePath();
              ctx.clip();
              ctx.fillStyle = bgColor;
              ctx.fillRect(0, 0, 600, 600);
              ctx.drawImage(img, 160, 160, 280, 280);
              ctx.restore();
            } else if (bgShape === "octagon") {
              const c = 160;
              ctx.save();
              ctx.beginPath();
              ctx.moveTo(c, 0);
              ctx.lineTo(600 - c, 0);
              ctx.lineTo(600, c);
              ctx.lineTo(600, 600 - c);
              ctx.lineTo(600 - c, 600);
              ctx.lineTo(c, 600);
              ctx.lineTo(0, 600 - c);
              ctx.lineTo(0, c);
              ctx.closePath();
              ctx.clip();
              ctx.fillStyle = bgColor;
              ctx.fillRect(0, 0, 600, 600);
              ctx.drawImage(img, 60, 60, 480, 480);
              ctx.restore();
            } else {
              ctx.fillStyle = bgColor;
              ctx.fillRect(0, 0, 600, 600);
              ctx.drawImage(img, 0, 0, 600, 600);
            }
            drawGlobalCenterOverlay(ctx, 300, 300, baseQrCanvasSize);
          }

          qrImagePng = canvas.toDataURL("image/png");

          const jpgCanvas = document.createElement("canvas");
          jpgCanvas.width = canvas.width;
          jpgCanvas.height = canvas.height;
          const jctx = jpgCanvas.getContext("2d");
          if (!jctx) {
            throw new Error("JPEG export canvas context could not be created.");
          }
          jctx.fillStyle = "#FFFFFF";
          jctx.fillRect(0, 0, jpgCanvas.width, jpgCanvas.height);
          jctx.drawImage(canvas, 0, 0);

          qrImageJpg = jpgCanvas.toDataURL("image/jpeg", 1.0);
          generatedPayload = finalData;
          generatedLabel = getGeneratedLabel();
          if (!printTitle.trim() || printTitle.trim() === generatedLabel.trim()) {
            printTitle = generatedLabel;
          }
          generatedAt = new Date().toLocaleString();
          if (manualGenerationRequested) {
            rememberGeneration(finalData, generatedLabel);
            manualGenerationRequested = false;
          }
        } catch (e: unknown) {
          console.error(e);
          showSaveToastMessage("Error generating QR preview: " + getErrorMessage(e), "error");
        } finally {
          loading = false;
        }
      };
      img.onerror = () => {
        showSaveToastMessage("Error generating QR preview: generated image could not be loaded.", "error");
        loading = false;
        manualGenerationRequested = false;
      };
      img.src = rustImageB64;
    } catch (e: unknown) {
      console.error(e);
      showSaveToastMessage("Error generating QR: " + getErrorMessage(e), "error");
      loading = false;
      manualGenerationRequested = false;
    }
  }

  function buildFinalQrData() {
    let finalData = qrData;
    if (dataType === "WiFi") {
      finalData = `WIFI:S:${wifiSsid};T:WPA;P:${wifiPass};;`;
    } else if (dataType === "DogTag") {
      finalData = `PET:${petName}\nCHIP:${microchipNum}\nOWNER:${ownerName}\nTEL:${ownerPhone}\nADDR:${ownerAddr}`;
    } else if (dataType === "vCard") {
      finalData = `BEGIN:VCARD\nVERSION:3.0\nN:${vCardLast};${vCardFirst}\nFN:${vCardFirst} ${vCardLast}\nORG:${vCardOrg}\nTEL:${vCardPhone}\nEMAIL:${vCardEmail}\nEND:VCARD`;
    } else if (dataType === "Email") {
      finalData = `mailto:${emailTo}?subject=${encodeURIComponent(emailSub)}&body=${encodeURIComponent(emailBody)}`;
    } else if (dataType === "SMS") {
      finalData = `smsto:${smsPhone}:${smsMsg}`;
    } else if (dataType === "Phone") {
      finalData = `tel:${phoneNum}`;
    } else if (dataType === "Geo") {
      finalData = `geo:${geoLat},${geoLng}`;
    } else if (dataType === "WhatsApp") {
      finalData = `https://wa.me/${waPhone.replace(/\D/g, '')}?text=${encodeURIComponent(waMsg)}`;
    } else if (dataType === "Crypto") {
      finalData = buildCryptoPayload();
    } else if (dataType === "Text") {
      finalData = qrData;
    } else if (dataType === "Event") {
      const cleanDate = (d: string) => {
        if (!d) return "";
        return d.replace(/[-:]/g, "") + "00";
      };
      finalData = `BEGIN:VCALENDAR\nVERSION:2.0\nBEGIN:VEVENT\nSUMMARY:${eventTitle}\nDTSTART:${cleanDate(eventStart)}\nDTEND:${cleanDate(eventEnd)}\nLOCATION:${eventLoc}\nEND:VEVENT\nEND:VCALENDAR`;
    } else if (dataType === "Social") {
      if (socialPlatform === "Facebook") finalData = `https://facebook.com/${socialUser}`;
      else if (socialPlatform === "Instagram") finalData = `https://instagram.com/${socialUser}`;
      else if (socialPlatform === "Twitter") finalData = `https://twitter.com/${socialUser}`;
    } else if (dataType === "LinkedIn") {
      finalData = `https://linkedin.com/in/${linkedinUser}`;
    } else if (dataType === "YouTube") {
      const h = ytHandle.startsWith("@") ? ytHandle : "@" + ytHandle;
      finalData = `https://youtube.com/${h}`;
    } else if (dataType === "TikTok") {
      const u = tiktokUser.startsWith("@") ? tiktokUser : "@" + tiktokUser;
      finalData = `https://tiktok.com/${u}`;
    } else if (dataType === "Zoom") {
      finalData = `https://zoom.us/j/${zoomMeetingId}${zoomPass ? "?pwd=" + zoomPass : ""}`;
    }

    return finalData;
  }

  function buildQrRenderOptions(finalData: string) {
    return {
      data: finalData,
      color1,
      color2,
      color3,
      color4: useFourthStop ? color4 : undefined,
      bgColor,
      eyeOut,
      eyeIn,
      fillType,
      mainShape,
      bgShape,
      eyeShape,
      logoB64: logoBase64 || undefined,
      logoSize: logoSizePercent,
      logoOpacity: logoOpacityPercent,
      enableFrame,
      frameText,
      frameTextTop,
      frameTextMode,
      frameTextTopMode,
      frameTextRadius,
      frameTextTopRadius,
      frameTextSpacing,
      frameTextTopSpacing,
      frameTextSize,
      frameTextTopSize,
      frameTextColor,
      frameTextTopColor,
      matchTextStyle,
      matchTextTopStyle,
      transparentTextBg,
      transparentTextTopBg,
      transparentFrameBg,
      ringStyle,
      ringColor,
      ringColor2,
      ringColor3,
      ringColor4,
      ringUseFourthStop,
      ringGradientMode,
      ringColorMode,
      centerOverlayMode,
      centerOverlayStyle,
      centerOverlayColor,
      centerOverlayColor2,
      centerOverlayColor3,
      centerOverlayColor4,
      centerOverlayUseFourthStop,
      centerOverlayGradientMode,
      centerOverlayColorMode
    };
  }

  $: if (typeof document !== 'undefined') {
    if (isScanning) {
      document.body.classList.add('scanning-active');
    } else {
      document.body.classList.remove('scanning-active');
    }
  }

  $: scannedResultCanOpen = canOpenResult(scannedResult);
</script>

{#if currentView === 'studio'}
  <main class="mobile-app">
    <input
      bind:this={fallbackColorInput}
      type="color"
      value={getActiveColorValue()}
      on:input={onStudioColorInput}
      class="native-color-proxy"
      aria-hidden="true"
      tabindex="-1"
    />
    {#if isScanning}
    <div class="scanner-overlay">
      <div class="scanner-header">
        <h2>SCANNING...</h2>
        <p>Point camera at a QR code or barcode</p>
      </div>
      <div class="scanner-stage">
        <div class="scanner-target">
          <div class="scanner-corner top-left"></div>
          <div class="scanner-corner top-right"></div>
          <div class="scanner-corner bottom-left"></div>
          <div class="scanner-corner bottom-right"></div>
          <div class="scanner-line"></div>
        </div>
        <p class="scanner-caption">Reads QR, Code 128, EAN, UPC, ISBN, Code 39, PDF417 and more</p>
      </div>
      <button class="cancel-scan-btn" on:click={cancelScanner}>CANCEL SCAN</button>
    </div>
  {:else}
    
  {#if showDogTagWarning}
    <div class="custom-modal-overlay">
      <div class="custom-modal warning-modal">
          <div class="modal-kicker">Legal warning</div>
          <h3>Generate Without Microchip?</h3>
          <p>Microchipping is legally required for all dogs in Ireland.</p>
          <p>Are you sure you want to generate this tag without a 15-digit microchip number?</p>
          <div class="modal-actions">
            <button class="modal-btn outline" on:click={() => showDogTagWarning = false}>Cancel</button>
            <button class="modal-btn danger" on:click={runGeneration}>Proceed Anyway</button>
          </div>
      </div>
    </div>
  {/if}

  {#if showScanDecryptModal}
    <div class="custom-modal-overlay">
      <div class="custom-modal decrypt-modal">
        <span class="modal-kicker">Encrypted QR</span>
        <h3>Unlock Payload</h3>
        {#if getPayloadAlgorithmLabel(scanEncryptedPayload)}
          <div class="crypto-meta">Algorithm: {getPayloadAlgorithmLabel(scanEncryptedPayload)}</div>
        {/if}
        <p>Enter the passphrase to decrypt this QR locally.</p>
        <div class="passphrase-field modal-passphrase">
          <input
            type={showScanDecryptPassphrase ? "text" : "password"}
            bind:value={scanDecryptPassphrase}
            placeholder="Passphrase"
            autocomplete="current-password"
            on:keydown={(event) => {
              if (event.key === "Enter") decryptScannedEncryptedQr();
              if (event.key === "Escape") closeScanDecryptModal();
            }}
          />
          <button type="button" class="reveal-btn" on:click={() => showScanDecryptPassphrase = !showScanDecryptPassphrase}>
            {showScanDecryptPassphrase ? "Hide" : "Show"}
          </button>
        </div>
        {#if scanDecryptError}
          <p class="modal-error">{scanDecryptError}</p>
        {/if}
        {#if scanDecryptedPlaintext && showScanDecryptSuccessBanner}
          <div class="modal-success">
            <span>✓</span>
            <strong>Decrypted</strong>
            <button
              class="modal-success-dismiss"
              type="button"
              aria-label="Dismiss decrypted banner"
              title="Dismiss"
              on:click={hideScanDecryptSuccessBanner}
            >
              x
            </button>
          </div>
        {/if}
        {#if scanDecryptedPlaintext}
          <div class={`forensic-date ${scanDecryptedForensicDate ? "verified" : "missing"}`}>
            {scanDecryptedForensicDate
              ? `Verified forensic date: ${scanDecryptedForensicDate}`
              : "No forensic date found in this encrypted payload"}
          </div>
          <div class="decrypted-output modal-decrypted">{scanDecryptedPlaintext}</div>
        {/if}
        <div class="modal-actions">
          <button class="modal-btn outline" type="button" on:click={closeScanDecryptModal}>Close</button>
          {#if scanDecryptedPlaintext && canOpenResult(scanDecryptedPlaintext)}
            <button class="modal-btn primary-modal" type="button" on:click={() => openLink(scanDecryptedPlaintext)}>Open Link</button>
          {:else}
            <button
              class:danger={!scanDecryptedPlaintext}
              class:success-action={Boolean(scanDecryptedPlaintext)}
              class="modal-btn"
              type="button"
              disabled={cryptoBusy || Boolean(scanDecryptedPlaintext)}
              on:click={decryptScannedEncryptedQr}
            >
              {cryptoBusy ? "Decrypting..." : scanDecryptedPlaintext ? "Verified" : "Decrypt"}
            </button>
          {/if}
        </div>
        {#if scanDecryptedPlaintext}
          <div class="modal-actions single-action">
            <button class="modal-btn outline copy-plaintext-btn" type="button" on:click={() => copyText(scanDecryptedPlaintext)}>
              <span class="copy-icon" aria-hidden="true"></span>
              <span>Copy Plaintext</span>
            </button>
            <button class="modal-btn outline" type="button" on:click={retryScanDecryption}>Decrypt Again</button>
          </div>
        {/if}
      </div>
    </div>
  {/if}

  {#if showCropModal}
      <div class="custom-modal-overlay">
        <div class="custom-modal crop-modal">
          <h3>📐 Position Logo</h3>
          <p>Drag and zoom to perfectly frame your logo.</p>
          <div class="crop-container" role="presentation"
            on:mousedown={startDrag}
            on:mousemove={onDrag}
            on:mouseup={stopDrag}
            on:mouseleave={stopDrag}
            on:touchstart={startDrag}
            on:touchmove={onDrag}
            on:touchend={stopDrag}
          >
            <img
              bind:this={cropImgEl}
              src={cropRawSrc}
              on:load={onCropImgLoad}
              class="crop-img"
              draggable="false"
              alt="Logo to crop"
            />
            {#if cropImgNaturalW > 0}
              {@const rect = cropImgEl?.getBoundingClientRect()}
              {@const scaleX = rect ? rect.width / cropImgNaturalW : 1}
              {@const scaleY = rect ? rect.height / cropImgNaturalH : 1}
              <div class="crop-box" style="
                left: {cropX * scaleX}px;
                top: {cropY * scaleY}px;
                width: {cropSize * scaleX}px;
                height: {cropSize * scaleY}px;
              "></div>
            {/if}
          </div>

          <div class="zoom-row">
            <span class="zoom-label">🔍 Zoom</span>
            <input type="range" min="1" max="4" step="0.05" value={cropZoom} on:input={onZoomChange} class="zoom-slider" />
            <span class="zoom-value">{cropZoom.toFixed(1)}x</span>
          </div>

          <div class="modal-actions" style="margin-top: 16px;">
            <button class="modal-btn outline" on:click={cancelCrop}>Cancel</button>
            <button class="modal-btn danger" on:click={commitCrop}>Use Logo</button>
          </div>
        </div>
      </div>
    {/if}

    <header class="lovely-header">
      <div class="logo-area">
        <div class="logo-icon">
          <img src="/app-icon.png" alt="QR Studio Ultra logo" />
        </div>
        <div class="logo-text">
          <h1>QR STUDIO <span class="ultra">ULTRA</span></h1>
          <p class="pbess">BY CYPHER SHADOWBOURNE</p>
        </div>
      </div>
      <button class="settings-nav-btn" on:click={() => currentView = 'settings'} title="Settings">
        ⚙️
      </button>
    </header>

    <div class="scrolling-content">
      <button class="activate-scan-btn" on:click={startScan}>📸 SCAN A QR CODE</button>

      {#if scannedResult}
        <fieldset class="panel result-panel">
          <legend class="result-legend">Scanned Result</legend>
          <div class="scan-meta">
            <span>{scannedKind || "Scan"}</span>
            {#if scannedFormat}
              <span>{scannedFormat}</span>
            {/if}
          </div>
          <div class="scanned-text-box">{scannedResult}</div>
          <div class="result-actions">
            {#if scannedResultCanOpen}
              <button class="result-btn primary-btn" on:click={() => openLink(scannedResult)}>OPEN</button>
            {/if}
            {#if isEncryptedQrPayload(scannedResult)}
              <button class="result-btn primary-btn" on:click={loadScannedEncryptedPayload}>DECRYPT</button>
            {/if}
            <button class="result-btn secondary-btn" on:click={() => copyText(scannedResult)}>COPY TEXT</button>
            <button class="result-btn cancel-btn" on:click={() => { scannedResult = ""; scannedFormat = ""; scannedKind = ""; }}>CLEAR</button>
          </div>
        </fieldset>
      {/if}

      <fieldset class="panel">
        <div class="panel-header-row" style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
          <legend style="margin-bottom: 0;">1. Content Data</legend>
          <button class="mini-action" type="button" style="background: var(--accent-gradient); color: white; border: none; padding: 4px 10px; border-radius: 8px; font-weight: bold; cursor: pointer; display: flex; align-items: center; gap: 4px;" on:click={() => showAiMagic = !showAiMagic}>
            ✨ AI Magic
          </button>
          <button class="mini-action" type="button" style="background: rgba(255,255,255,0.1); color: white; border: 1px solid rgba(255,255,255,0.2); padding: 4px 10px; border-radius: 8px; font-weight: bold; cursor: pointer; display: flex; align-items: center; gap: 4px;" on:click={() => currentView = 'settings'}>
            ⚙️ Settings
          </button>
        </div>

        {#if showAiMagic}
          <div class="ai-magic-container" transition:fade>
            <div class="ai-provider-row">
              <span class="ai-provider-badge">{getProviderLabel(aiMagicActiveProvider)}</span>
              <span class="ai-provider-note">Fallbacks use any saved provider keys.</span>
            </div>
            <textarea
              bind:value={aiMagicPrompt}
              placeholder="Describe the QR code you want... (e.g. 'Cyberpunk theme with neon pink and cyan colors')"
              rows="3"
              class="text-area ai-prompt-textarea"
            ></textarea>
            <div class="ai-chips">
              {#each aiExamples as example}
                <button class="ai-chip" type="button" on:click={() => aiMagicPrompt = example}>
                  {example}
                </button>
              {/each}
            </div>
            <label class="ai-force-center checkbox-label">
              <input type="checkbox" bind:checked={aiMagicForceCenterImage} />
              Force center image prompt
            </label>
            {#if aiMagicLastCenterImagePrompt}
              <div class="ai-center-prompt">
                <strong>Last center image prompt</strong>
                <span>{aiMagicLastCenterImagePrompt}</span>
              </div>
            {/if}
            <div class="ai-actions" style="display: flex; gap: 10px;">
              <button
                class="wallet-btn ai-generate-btn"
                type="button"
                disabled={aiMagicLoading || !aiMagicPrompt}
                on:click={() => handleAiMagicGenerate(false)}
              >
                {aiMagicLoading ? "Applying Magic..." : "Generate ✨"}
              </button>
              <button
                class="wallet-btn ai-generate-btn crazier-btn"
                type="button"
                style="background: linear-gradient(135deg, #ff0080, #7928ca); border: none;"
                disabled={aiMagicLoading || !aiMagicPrompt}
                on:click={() => handleAiMagicGenerate(true)}
              >
                {aiMagicLoading ? "Going Crazy..." : "Make it Crazier! 🚀"}
              </button>
            </div>
            {#if !hasAnyAiProviderKey()}
              <div class="ai-warning" style="margin-top: 10px; padding: 10px; background: rgba(255, 165, 0, 0.1); border: 1px solid rgba(255, 165, 0, 0.3); border-radius: 8px; color: #ffa500; font-size: 13px; display: flex; align-items: center; gap: 8px;">
                <span>⚠️</span>
                <span>No AI provider key is saved. <button style="background: none; border: none; color: #fff; text-decoration: underline; cursor: pointer; padding: 0; font-weight: bold;" on:click={() => currentView = 'settings'}>Add one in Settings</button> for AI Magic.</span>
              </div>
            {/if}
          </div>
        {/if}

        <select bind:value={dataType}>
          <option value="URL">Standard Link (URL)</option>
          <option value="Text">Plain Text</option>
          <option value="WiFi">WiFi Network</option>
          <option value="vCard">Contact Card (vCard)</option>
          <option value="Email">Send Email</option>
          <option value="SMS">Send SMS</option>
          <option value="Phone">Phone Call</option>
          <option value="Geo">Location (Coordinates)</option>
          <option value="DogTag">Dog Tag (Irish Law)</option>
          <option value="WhatsApp">WhatsApp Message</option>
          <option value="Crypto">Crypto Wallet Address</option>
          <option value="Event">Calendar Event (iCal)</option>
          <option value="Social">Social Profile</option>
          <option value="LinkedIn">LinkedIn Profile</option>
          <option value="YouTube">YouTube Channel</option>
          <option value="TikTok">TikTok Handle</option>
          <option value="Zoom">Zoom Meeting</option>
        </select>

        {#if dataType === "URL"}
          <input type="text" bind:value={qrData} placeholder="https://example.com" />
        {:else if dataType === "Text"}
          <textarea bind:value={qrData} placeholder="Type or paste your text here..." rows="3" class="text-area"></textarea>
        {:else if dataType === "WiFi"}
          <input type="text" bind:value={wifiSsid} placeholder="Network Name (SSID)" />
          <input type="password" bind:value={wifiPass} placeholder="Password" />
        {:else if dataType === "vCard"}
          <div class="row split">
            <input type="text" bind:value={vCardFirst} placeholder="First Name" />
            <input type="text" bind:value={vCardLast} placeholder="Last Name" />
          </div>
          <input type="text" bind:value={vCardOrg} placeholder="Company / Organization" />
          <input type="text" bind:value={vCardPhone} placeholder="Phone Number" />
          <input type="text" bind:value={vCardEmail} placeholder="Email Address" />
        {:else if dataType === "Email"}
          <input type="text" bind:value={emailTo} placeholder="To: (Email Address)" />
          <input type="text" bind:value={emailSub} placeholder="Subject Line" />
          <textarea bind:value={emailBody} placeholder="Email Body" rows="3" class="text-area"></textarea>
        {:else if dataType === "SMS"}
          <input type="text" bind:value={smsPhone} placeholder="Phone Number" />
          <textarea bind:value={smsMsg} placeholder="Message Content" rows="3" class="text-area"></textarea>
        {:else if dataType === "Phone"}
          <input type="text" bind:value={phoneNum} placeholder="Phone Number" />
        {:else if dataType === "Geo"}
          <div class="row split">
            <input type="text" bind:value={geoLat} placeholder="Latitude (e.g., 53.3498)" />
            <input type="text" bind:value={geoLng} placeholder="Longitude (e.g., -6.2603)" />
          </div>
        {:else if dataType === "DogTag"}
          <input type="text" bind:value={petName} placeholder="Pet Name" />
          <input type="text" bind:value={microchipNum} placeholder="Microchip Number (15 Digits)" />
          <input type="text" bind:value={ownerName} placeholder="Owner Name" />
          <input type="text" bind:value={ownerPhone} placeholder="Phone Number" />
          <textarea bind:value={ownerAddr} placeholder="Full Address" rows="3" class="text-area"></textarea>
        {:else if dataType === "WhatsApp"}
          <input type="text" bind:value={waPhone} placeholder="WhatsApp Phone (incl. Country Code)" />
          <textarea bind:value={waMsg} placeholder="Pre-filled Message (Optional)" rows="2" class="text-area"></textarea>
        {:else if dataType === "Crypto"}
          <input type="text" bind:value={walletName} placeholder="Wallet Profile Name" />
          <div class="row split wallet-row">
            <select bind:value={cryptoType} class="outline-select">
              <option value="bitcoin">Bitcoin (BTC)</option>
              <option value="ethereum">Ethereum (ETH)</option>
              <option value="litecoin">Litecoin (LTC)</option>
              <option value="dogecoin">Dogecoin (DOGE)</option>
              <option value="solana">Solana (SOL)</option>
            </select>
            <input type="text" bind:value={cryptoAddr} placeholder="Wallet Address" />
          </div>
          <div class="row split wallet-row">
            <input type="text" bind:value={cryptoAmount} placeholder="Amount (Optional)" />
            <input type="text" bind:value={cryptoLabel} placeholder="Label / Recipient" />
          </div>
          <textarea bind:value={cryptoMessage} placeholder="Message or memo (optional)" rows="2" class="text-area"></textarea>
          <div class="wallet-toolbar">
            <button class="wallet-btn wallet-save-btn" type="button" on:click={saveCurrentWalletProfile}>SAVE WALLET PROFILE</button>
            <div class="wallet-payload-preview">{buildCryptoPayload()}</div>
          </div>
          {#if savedWallets.length}
            <div class="sub-panel wallet-library">
              <p class="sub-label">Saved Wallets</p>
              {#each savedWallets as wallet (wallet.id)}
                <div class="wallet-card">
                  <div class="wallet-card-copy">
                    <strong>{wallet.name}</strong>
                    <div class="wallet-meta">{toDisplayCase(wallet.network)} • {wallet.address}</div>
                  </div>
                  <div class="wallet-card-actions">
                    <button class="wallet-btn" type="button" on:click={() => loadWalletProfile(wallet)}>LOAD</button>
                    <button class="wallet-btn wallet-delete-btn" type="button" on:click={() => deleteWalletProfile(wallet.id)}>DELETE</button>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        {:else if dataType === "Event"}
          <input type="text" bind:value={eventTitle} placeholder="Event Title / Summary" />
          <div class="row split">
            <label>Start: <input type="datetime-local" bind:value={eventStart} /></label>
            <label>End: <input type="datetime-local" bind:value={eventEnd} /></label>
          </div>
          <input type="text" bind:value={eventLoc} placeholder="Location" />
        {:else if dataType === "Social"}
          <div class="row split">
            <select bind:value={socialPlatform} class="outline-select">
              <option value="Facebook">Facebook</option>
              <option value="Instagram">Instagram</option>
              <option value="Twitter">Twitter (X)</option>
            </select>
            <input type="text" bind:value={socialUser} placeholder="Username / Handle" />
          </div>
        {:else if dataType === "LinkedIn"}
          <input type="text" bind:value={linkedinUser} placeholder="LinkedIn Username (e.g., pbess)" />
        {:else if dataType === "YouTube"}
          <input type="text" bind:value={ytHandle} placeholder="Channel Name or @Handle" />
        {:else if dataType === "TikTok"}
          <input type="text" bind:value={tiktokUser} placeholder="@Username" />
        {:else if dataType === "Zoom"}
          <div class="row split">
            <input type="text" bind:value={zoomMeetingId} placeholder="Meeting ID" />
            <input type="text" bind:value={zoomPass} placeholder="Passcode (Optional)" />
          </div>
        {/if}
      </fieldset>

      <fieldset class="panel intelligence-panel">
        <legend>Studio Intelligence</legend>
        <div class={`scan-score-card ${currentScannability.className}`}>
          <div class="scan-score-head">
            <div>
              <div class="scan-score-label">Scannability</div>
              <strong>{currentScannability.label}</strong>
            </div>
            <span>{currentScannability.score}%</span>
          </div>
          <div class="scan-score-bar">
            <span style={`width: ${currentScannability.score}%`}></span>
          </div>
          <p>{currentScannability.notes[0]}</p>
        </div>

        <div class="sub-panel">
          <div class="field-head">
            <p class="sub-label">Templates</p>
            <button class="mini-action" type="button" on:click={saveCurrentTemplate}>Save</button>
          </div>
          <input type="text" bind:value={templateName} placeholder="Template name" />
          {#if savedTemplates.length}
            <div class="compact-list">
              {#each savedTemplates as template (template.id)}
                <div class="compact-card">
                  <div>
                    <strong>{template.name}</strong>
                    <span>{template.kind === "batch" || getTemplateBatchLines(template).length ? "Batch" : "Template"} · {template.createdAt}</span>
                  </div>
                  <div class="compact-actions">
                    <button class="mini-action" type="button" on:click={() => loadTemplate(template)}>Load</button>
                    <button class="mini-action danger-mini" type="button" on:click={() => deleteTemplate(template.id)}>Delete</button>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>

        <div class="sub-panel">
          <div class="field-head">
            <p class="sub-label">Batch Generation</p>
            <span class="batch-count">{getBatchLines().length}/50</span>
          </div>
          <textarea
            bind:value={batchInput}
            rows="4"
            class="text-area"
            placeholder="One QR payload per line (URLs, text, etc)..."
          ></textarea>
          <div class="row split mb-10" style="position: relative;">
            <button class="wallet-btn batch-btn" type="button" style="flex: 2; overflow: hidden;" disabled={batchBusy} on:click={generateBatch}>
              {#if batchBusy}
                <div class="progress-bar-bg">
                  <div class="progress-bar-fill" style={`width: ${(batchProgress / batchTotal) * 100}%`}></div>
                </div>
                <span style="position: relative; z-index: 1;">{batchProgress}/{batchTotal} Generating...</span>
              {:else}
                Generate Batch
              {/if}
            </button>
            <button class="mini-action danger-mini" type="button" style="height: 44px; border-radius: 10px;" on:click={() => batchInput = ""}>Clear</button>
          </div>
          <button class="wallet-btn batch-template-btn" type="button" on:click={saveCurrentBatchTemplate}>
            Save Batch Template
          </button>
          {#if batchResults.length}
            <div class="field-head mt-10">
              <p class="sub-label" style="margin-bottom: 0;">Results ({batchResults.length})</p>
              <div style="display: flex; gap: 8px;">
                <button class="mini-action" type="button" on:click={saveAllBatchAsZip}>Save All (ZIP)</button>
                <button class="mini-action" type="button" on:click={clearBatchResults}>Clear Results</button>
              </div>
            </div>
            <div class="batch-grid">
              {#each batchResults as item, i (item.id)}
                <div 
                  class="batch-card"
                  in:scale={{ duration: 400, delay: Math.min(i * 30, 600), easing: cubicOut, start: 0.8 }}
                >
                  <img src={item.image} alt={`${item.label} QR`} />
                  <div class="batch-card-copy">
                    <strong>{item.label}</strong>
                    <span>{item.score}%</span>
                  </div>
                  <div class="compact-actions">
                    <button class="mini-action" type="button" on:click={() => loadBatchResult(item)}>Load</button>
                    <button class="mini-action" type="button" on:click={() => saveBatchResult(item)}>Save</button>
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </fieldset>

      <fieldset class="panel crypto-panel">
        <legend>Encrypt / Decrypt</legend>

        <div class="sub-panel crypto-tool">
          <div class="field-head">
            <p class="sub-label">Encrypt</p>
            <button class="help-dot" type="button" title={`Uses Argon2id for key derivation, then ${encryptionAlgorithm === "aes-256-gcm" ? "AES-256-GCM" : "ChaCha20-Poly1305"} for authenticated encryption.`}>?</button>
          </div>
          <textarea
            bind:value={encryptPlaintext}
            placeholder="Plaintext to protect..."
            rows="3"
            class="text-area"
            autocomplete="off"
          ></textarea>
          <div class="passphrase-field">
            <input
              type={showEncryptPassphrase ? "text" : "password"}
              bind:value={encryptPassphrase}
              placeholder="Passphrase"
              autocomplete="new-password"
            />
            <button type="button" class="reveal-btn" on:click={() => showEncryptPassphrase = !showEncryptPassphrase} title={showEncryptPassphrase ? "Hide passphrase" : "Show passphrase"}>
              {showEncryptPassphrase ? "Hide" : "Show"}
            </button>
          </div>
          <div class={`strength-meter ${encryptStrength.className}`}>
            <span style={`width: ${encryptStrength.percent}%`}></span>
            <strong>{encryptStrength.label}</strong>
          </div>
          <div class="algorithm-toggle" role="group" aria-label="Encryption algorithm">
            <button
              class:active={encryptionAlgorithm === "chacha20-poly1305"}
              type="button"
              on:click={() => encryptionAlgorithm = "chacha20-poly1305"}
            >
              ChaCha20
            </button>
            <button
              class:active={encryptionAlgorithm === "aes-256-gcm"}
              type="button"
              on:click={() => encryptionAlgorithm = "aes-256-gcm"}
            >
              AES-256-GCM
            </button>
            <button class="help-dot algorithm-help" type="button" title="Both modes are standard authenticated encryption. AES-256-GCM is widely accelerated on many CPUs; ChaCha20-Poly1305 is a strong default on mobile and general devices.">?</button>
          </div>
          <button class="wallet-btn crypto-action-btn" type="button" disabled={cryptoBusy} on:click={encryptPlaintextForQr}>
            ENCRYPT & GENERATE QR
          </button>
          {#if encryptedPayload}
            <div class="forensic-date">Forensic date: {encryptedForensicDate}</div>
            <div class="encrypted-output">{encryptedPayload}</div>
            <div class="result-actions">
              <button class="result-btn secondary-btn" type="button" on:click={() => copyText(encryptedPayload)}>COPY PAYLOAD</button>
              <button class="result-btn primary-btn" type="button" on:click={() => useEncryptedPayloadForQr(encryptedPayload)}>GENERATE QR</button>
            </div>
          {/if}
        </div>

        <div class="sub-panel crypto-tool">
          <div class="field-head">
            <p class="sub-label">Decrypt</p>
            <button class="help-dot" type="button" title="Paste or scan a QRU1 payload. The app reads the algorithm from the payload and decrypts locally.">?</button>
          </div>
          <textarea
            bind:value={decryptPayload}
            placeholder="Paste encrypted QR payload..."
            rows="3"
            class="text-area"
            autocomplete="off"
          ></textarea>
          <div class="passphrase-field">
            <input
              type={showDecryptPassphrase ? "text" : "password"}
              bind:value={decryptPassphrase}
              placeholder="Passphrase"
              autocomplete="current-password"
            />
            <button type="button" class="reveal-btn" on:click={() => showDecryptPassphrase = !showDecryptPassphrase} title={showDecryptPassphrase ? "Hide passphrase" : "Show passphrase"}>
              {showDecryptPassphrase ? "Hide" : "Show"}
            </button>
          </div>
          <button class="wallet-btn crypto-action-btn" type="button" disabled={cryptoBusy} on:click={decryptEncryptedQrPayload}>
            DECRYPT PAYLOAD
          </button>
          {#if decryptedPlaintext}
            <div class={`forensic-date ${decryptedForensicDate ? "verified" : "missing"}`}>
              {decryptedForensicDate
                ? `Verified forensic date: ${decryptedForensicDate}`
                : "No forensic date found in this encrypted payload"}
            </div>
            <div class="decrypted-output">{decryptedPlaintext}</div>
            <div class="result-actions">
              <button class="result-btn secondary-btn" type="button" on:click={() => copyText(decryptedPlaintext)}>COPY TEXT</button>
            </div>
          {/if}
        </div>
      </fieldset>

      <fieldset class="panel">
        <legend>2. Body & Colors</legend>
        <div class="row split mb-10">
          <label class="select-field full-width">Outer Shape
            <select bind:value={bgShape} class="outline-select">
              <option value="square">Square</option>
              <option value="circle">Round Sticker</option>
              <option value="rounded">Rounded Square</option>
            </select>
          </label>
        </div>

        <div class="row split">
          <label class="select-field">Inner Block Style
            <select bind:value={mainShape}>
              <option value="square">Square Blocks</option>
              <option value="circle">Dots (Circles)</option>
              <option value="rounded">Rounded Blocks</option>
            </select>
          </label>
          <label class="select-field">Fill Mode
            <select bind:value={fillType}>
              <option value="Solid">Solid Color</option>
              <option value="Linear">Gradient</option>
            </select>
          </label>
        </div>
        
        <div class="row split color-row mt-10">
          <label>Main <input type="color" bind:value={color1} /> <input type="text" bind:value={color1} class="hex-input" on:blur={(event) => onHexBlur('color1', event)}/> <button class="mini-color-btn" type="button" on:click={() => pickColorInto('color1')}>Pick</button></label>
          <label>Mid <input type="color" bind:value={color2} /> <input type="text" bind:value={color2} class="hex-input" on:blur={(event) => onHexBlur('color2', event)}/> <button class="mini-color-btn" type="button" on:click={() => pickColorInto('color2')}>Pick</button></label>
          <label>End <input type="color" bind:value={color3} /> <input type="text" bind:value={color3} class="hex-input" on:blur={(event) => onHexBlur('color3', event)}/> <button class="mini-color-btn" type="button" on:click={() => pickColorInto('color3')}>Pick</button></label>
        </div>
        <label class="checkbox-label" style="font-size: 0.85rem; margin: 10px 0 8px;">
          <input type="checkbox" bind:checked={useFourthStop} /> 4th Gradient Stop
        </label>
        {#if useFourthStop}
          <div class="row color-row">
            <label>Accent <input type="color" bind:value={color4} /> <input type="text" bind:value={color4} class="hex-input" on:blur={(event) => onHexBlur('color4', event)}/> <button class="mini-color-btn" type="button" on:click={() => pickColorInto('color4')}>Pick</button></label>
          </div>
        {/if}
        
        <div class="row color-row mt-10">
          <label>BG <input type="color" bind:value={bgColor} /> <input type="text" bind:value={bgColor} class="hex-input" on:blur={(event) => onHexBlur('bgColor', event)}/> <button class="mini-color-btn" type="button" on:click={() => pickColorInto('bgColor')}>Pick</button></label>
        </div>

        <div class="sub-panel">
          <p class="sub-label">Solid Colors</p>
          <div class="swatch-grid">
            {#each solidColors as color}
              <button class="swatch" type="button" style="background: {color};" on:click={() => applySolid(color)} aria-label={color}></button>
            {/each}
          </div>
        </div>

        <div class="sub-panel">
          <p class="sub-label">Gradients</p>
          <div class="preset-grid">
            {#each gradientPresets as preset}
              <button class="preset-btn" type="button" style="background: {gradientPreviewCss(preset.c1, preset.c2, preset.c3)};" on:click={() => applyGradient(preset.c1, preset.c2, preset.c3)}>
                {preset.name}
              </button>
            {/each}
          </div>
        </div>

        <div class="sub-panel">
          <p class="sub-label">Quick Pick</p>
          <p class="sub-note">Use any `Pick` button first, then tap a recent color here to reuse it on that field.</p>
          {#if recentColors.length}
            <div class="studio-swatches">
              {#each recentColors as color}
                <button class="swatch" type="button" style="background: {color};" on:click={() => applyColorChip(activeColorTarget, color)} aria-label={color}></button>
              {/each}
            </div>
          {/if}
        </div>
      </fieldset>

      <fieldset class="panel">
        <legend>3. Finder Patterns (Eyes)</legend>
        <label class="select-field full-width">Eye Style
          <select bind:value={eyeShape}>
            <option value="square">Square Eyes</option>
            <option value="circle">Circular Eyes</option>
            <option value="rounded">Rounded Eyes</option>
          </select>
        </label>
        <div class="row split color-row mt-10">
          <label>Outer
            <input type="color" bind:value={eyeOut} />
            <input type="text" bind:value={eyeOut} class="hex-input" on:blur={(event) => onHexBlur('eyeOut', event)}/>
            <button class="mini-color-btn" type="button" on:click={() => pickColorInto('eyeOut')}>Pick</button>
          </label>
          <label>Inner
            <input type="color" bind:value={eyeIn} />
            <input type="text" bind:value={eyeIn} class="hex-input" on:blur={(event) => onHexBlur('eyeIn', event)}/>
            <button class="mini-color-btn" type="button" on:click={() => pickColorInto('eyeIn')}>Pick</button>
          </label>
        </div>
      </fieldset>

      <fieldset class="panel pro-panel">
        <legend>4. PRO Overlay Frame</legend>
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={enableFrame} /> Enable Ring Overlay
        </label>
        <div class="row split mt-5">
          <label class="checkbox-label" style="font-size: 0.85rem; margin-bottom: 12px;">
            <input type="checkbox" bind:checked={transparentFrameBg} disabled={!enableFrame} /> Transparent Ring BG
          </label>
        </div>
        <div class="row split mt-5">
          <input type="text" bind:value={frameTextTop} placeholder="Top Text (Optional)" disabled={!enableFrame} />
        </div>

        <div class="sub-panel mt-10 mb-10" style="background: rgba(0,0,0,0.03); border-radius: 8px; padding: 10px;">
          <p class="sub-label" style="font-weight: bold; margin-bottom: 8px;">Top Text Settings</p>
          <div class="row split">
            <label class="checkbox-label" style="font-size: 0.85rem;">
              <input type="checkbox" bind:checked={transparentTextTopBg} disabled={!enableFrame} /> Transparent BG
            </label>
            <label class="checkbox-label" style="font-size: 0.85rem;">
              <input type="checkbox" bind:checked={matchTextTopStyle} disabled={!enableFrame} /> Match Style
            </label>
          </div>
          <div class="row split mt-5">
            <select bind:value={frameTextTopMode} disabled={!enableFrame} style="font-size: 0.8rem; height: 32px;">
              <option value="flat">Flat Text</option>
              <option value="curved">Conform to Shape</option>
            </select>
          </div>
          {#if frameTextTopMode === 'curved'}
          <div class="row mt-5">
            <label style="font-size: 0.75rem; flex:1;">Distance
              <input type="range" min="200" max="450" bind:value={frameTextTopRadius} disabled={!enableFrame} />
            </label>
            <label style="font-size: 0.75rem; flex:1;">Spacing
              <input type="range" min="-10" max="50" bind:value={frameTextTopSpacing} disabled={!enableFrame} />
            </label>
          </div>
          {/if}
          <div class="row split color-row mt-5">
            <label style="font-size: 0.75rem;">Size
              <input type="number" bind:value={frameTextTopSize} min="10" max="150" disabled={!enableFrame} style="width: 60px; height: 28px;"/>
            </label>
            <label style="font-size: 0.75rem;">Color
              <input type="color" bind:value={frameTextTopColor} disabled={!enableFrame}/>
            </label>
          </div>
        </div>

        <div class="row split mt-5">
          <input type="text" bind:value={frameText} placeholder="Bottom Text" disabled={!enableFrame} />
        </div>

        <div class="sub-panel mt-10 mb-10" style="background: rgba(0,0,0,0.03); border-radius: 8px; padding: 10px;">
          <p class="sub-label" style="font-weight: bold; margin-bottom: 8px;">Bottom Text Settings</p>
          <div class="row split">
            <label class="checkbox-label" style="font-size: 0.85rem;">
              <input type="checkbox" bind:checked={transparentTextBg} disabled={!enableFrame} /> Transparent BG
            </label>
            <label class="checkbox-label" style="font-size: 0.85rem;">
              <input type="checkbox" bind:checked={matchTextStyle} disabled={!enableFrame} /> Match Style
            </label>
          </div>
          <div class="row split mt-5">
            <select bind:value={frameTextMode} disabled={!enableFrame} style="font-size: 0.8rem; height: 32px;">
              <option value="flat">Flat Text</option>
              <option value="curved">Conform to Shape</option>
            </select>
          </div>
          {#if frameTextMode === 'curved'}
          <div class="row mt-5">
            <label style="font-size: 0.75rem; flex:1;">Distance
              <input type="range" min="200" max="450" bind:value={frameTextRadius} disabled={!enableFrame} />
            </label>
            <label style="font-size: 0.75rem; flex:1;">Spacing
              <input type="range" min="-10" max="50" bind:value={frameTextSpacing} disabled={!enableFrame} />
            </label>
          </div>
          {/if}
          <div class="row split color-row mt-5">
            <label style="font-size: 0.75rem;">Size
              <input type="number" bind:value={frameTextSize} min="10" max="150" disabled={!enableFrame} style="width: 60px; height: 28px;"/>
            </label>
            <label style="font-size: 0.75rem;">Color
              <input type="color" bind:value={frameTextColor} disabled={!enableFrame}/>
            </label>
          </div>
        </div>
        <div class="row split mt-10">
          <label class="select-field full-width">Outer Ring Style
            <select bind:value={ringStyle} disabled={!enableFrame}>
              <option value="none">No Ring (Overlay Only)</option>
              <option value="solid">Solid Ring</option>
              <option value="double">Double Ring</option>
              <option value="dotted">Dotted Ring (Dots)</option>
              <option value="dashed">Dashed Ring</option>
              <option value="rounded">Rounded Ring</option>
              <option value="diamond">Diamond Ring</option>
              <option value="gradient">Gradient Ring</option>
              <option value="neon">Neon Glow Ring</option>
            </select>
          </label>
        </div>
        <div class="row split color-row mt-10">
          <label>Ring
            <input type="color" bind:value={ringColor} disabled={!enableFrame}/>
            <input type="text" bind:value={ringColor} class="hex-input" on:blur={(event) => onHexBlur('ringColor', event)} disabled={!enableFrame}/>
            <button class="mini-color-btn" type="button" on:click={() => pickColorInto('ringColor')} disabled={!enableFrame}>Pick</button>
          </label>
          <label>Text
            <input type="color" bind:value={frameTextColor} disabled={!enableFrame}/>
            <input type="text" bind:value={frameTextColor} class="hex-input" on:blur={(event) => onHexBlur('frameTextColor', event)} disabled={!enableFrame}/>
            <button class="mini-color-btn" type="button" on:click={() => pickColorInto('frameTextColor')} disabled={!enableFrame}>Pick</button>
          </label>
        </div>
        <div class="sub-panel mt-10">
          <p class="sub-label">Overlay Gradient</p>
          <label class="select-field full-width">Outer Ring Fill
            <select bind:value={ringColorMode} disabled={!enableFrame || ringStyle === "none" || ringStyle === "solid"}>
              <option value="solid">Use Solid Ring Color</option>
              <option value="gradient">Use Gradient Colors</option>
            </select>
          </label>
          <label class="select-field full-width mt-10">Outer Gradient Source
            <select bind:value={ringGradientMode} disabled={!enableFrame || ringStyle === "none" || ringStyle === "solid" || ringColorMode !== "gradient"}>
              <option value="match-main">Match Main QR Gradient</option>
              <option value="custom">Custom Overlay Gradient</option>
            </select>
          </label>
          {#if ringStyle !== "none" && ringStyle !== "solid" && ringColorMode === "gradient" && ringGradientMode === "custom"}
            <div class="row split color-row mt-10">
              <label>Main
                <input type="color" bind:value={ringColor} disabled={!enableFrame}/>
                <input type="text" bind:value={ringColor} class="hex-input" on:blur={(event) => onHexBlur('ringColor', event)} disabled={!enableFrame}/>
                <button class="mini-color-btn" type="button" on:click={() => pickColorInto('ringColor')} disabled={!enableFrame}>Pick</button>
              </label>
              <label>Mid
                <input type="color" bind:value={ringColor2} disabled={!enableFrame}/>
                <input type="text" bind:value={ringColor2} class="hex-input" on:blur={(event) => onHexBlur('ringColor2', event)} disabled={!enableFrame}/>
                <button class="mini-color-btn" type="button" on:click={() => pickColorInto('ringColor2')} disabled={!enableFrame}>Pick</button>
              </label>
              <label>End
                <input type="color" bind:value={ringColor3} disabled={!enableFrame}/>
                <input type="text" bind:value={ringColor3} class="hex-input" on:blur={(event) => onHexBlur('ringColor3', event)} disabled={!enableFrame}/>
                <button class="mini-color-btn" type="button" on:click={() => pickColorInto('ringColor3')} disabled={!enableFrame}>Pick</button>
              </label>
            </div>
            <label class="checkbox-label" style="font-size: 0.85rem; margin: 10px 0 8px;">
              <input type="checkbox" bind:checked={ringUseFourthStop} disabled={!enableFrame} /> 4th Overlay Stop
            </label>
            {#if ringUseFourthStop}
              <div class="row color-row">
                <label>Accent
                  <input type="color" bind:value={ringColor4} disabled={!enableFrame}/>
                  <input type="text" bind:value={ringColor4} class="hex-input" on:blur={(event) => onHexBlur('ringColor4', event)} disabled={!enableFrame}/>
                  <button class="mini-color-btn" type="button" on:click={() => pickColorInto('ringColor4')} disabled={!enableFrame}>Pick</button>
                </label>
              </div>
            {/if}
          {/if}
        </div>
      </fieldset>

      <fieldset class="panel blank-panel">
        <input type="file" accept="image/png, image/jpeg" bind:this={fileInput} on:change={handleLogoUpload} style="display: none;" />
        <button class="upload-btn" on:click={triggerFileInput} class:has-logo={logoName !== ""}>
          {logoName !== "" ? `Logo Loaded: ${logoName}` : "Upload Center Logo"}
        </button>
        <div class="sub-panel mt-10">
          <p class="sub-label">Logo Controls</p>
          <label>
            Size
            <input type="range" min="10" max="36" step="1" bind:value={logoSizePercent} disabled={!logoBase64} />
            <span class="range-value">{logoSizePercent}%</span>
          </label>
          <div class="logo-safety-meter {logoSizePercent <= 24 ? 'safe' : logoSizePercent <= 30 ? 'warning' : 'danger'}">
            <span style="width: {(logoSizePercent / 36) * 100}%"></span>
          </div>
          <label class="mt-10">
            Opacity
            <input type="range" min="15" max="100" step="1" bind:value={logoOpacityPercent} disabled={!logoBase64} />
            <span class="range-value">{logoOpacityPercent}%</span>
          </label>
          <p class="sub-note">Bigger logos look strong, but staying under about 30% keeps scanning safer.</p>
        </div>
        <div class="sub-panel mt-10">
          <p class="sub-label">Center Photo Overlay</p>
          <label class="select-field full-width">Inner Overlay Mode
            <select bind:value={centerOverlayMode} disabled={!logoBase64}>
              <option value="none">No Inner Overlay</option>
              <option value="match" disabled={!enableFrame}>Match Outer Overlay</option>
              <option value="custom">Custom Inner Overlay</option>
            </select>
          </label>
          {#if centerOverlayMode === "custom"}
            <div class="row split mt-10">
              <label class="select-field full-width">Inner Ring Style
                <select bind:value={centerOverlayStyle} disabled={!logoBase64}>
                  <option value="solid">Solid Ring</option>
                  <option value="double">Double Ring</option>
                  <option value="dotted">Dotted Ring</option>
                  <option value="dashed">Dashed Ring</option>
                  <option value="rounded">Rounded Ring</option>
                  <option value="diamond">Diamond Ring</option>
                  <option value="gradient">Gradient Ring</option>
                  <option value="neon">Neon Glow Ring</option>
                </select>
              </label>
            </div>
            <label class="select-field full-width mt-10">Inner Ring Fill
              <select bind:value={centerOverlayColorMode} disabled={!logoBase64 || centerOverlayStyle === "none" || centerOverlayStyle === "solid"}>
                <option value="solid">Use Solid Inner Color</option>
                <option value="gradient">Use Gradient Colors</option>
              </select>
            </label>
            <label class="select-field full-width mt-10">Inner Gradient Source
              <select bind:value={centerOverlayGradientMode} disabled={!logoBase64 || centerOverlayStyle === "none" || centerOverlayStyle === "solid" || centerOverlayColorMode !== "gradient"}>
                <option value="match-outer">Match Outer Overlay</option>
                <option value="match-main">Match Main QR Gradient</option>
                <option value="custom">Custom Inner Gradient</option>
              </select>
            </label>
            {#if centerOverlayStyle === "solid" || centerOverlayColorMode === "solid" || centerOverlayGradientMode === "custom"}
              <div class="row split color-row mt-10">
                <label>Inner Ring <input type="color" bind:value={centerOverlayColor} disabled={!logoBase64}/> <input type="text" bind:value={centerOverlayColor} class="hex-input" on:blur={(event) => onHexBlur('centerOverlayColor', event)} disabled={!logoBase64}/> <button class="mini-color-btn" type="button" on:click={() => pickColorInto('centerOverlayColor')} disabled={!logoBase64}>Pick</button></label>
                {#if centerOverlayStyle !== "solid" && centerOverlayColorMode === "gradient" && centerOverlayGradientMode === "custom"}
                  <label>Mid <input type="color" bind:value={centerOverlayColor2} disabled={!logoBase64}/> <input type="text" bind:value={centerOverlayColor2} class="hex-input" on:blur={(event) => onHexBlur('centerOverlayColor2', event)} disabled={!logoBase64}/> <button class="mini-color-btn" type="button" on:click={() => pickColorInto('centerOverlayColor2')} disabled={!logoBase64}>Pick</button></label>
                  <label>End <input type="color" bind:value={centerOverlayColor3} disabled={!logoBase64}/> <input type="text" bind:value={centerOverlayColor3} class="hex-input" on:blur={(event) => onHexBlur('centerOverlayColor3', event)} disabled={!logoBase64}/> <button class="mini-color-btn" type="button" on:click={() => pickColorInto('centerOverlayColor3')} disabled={!logoBase64}>Pick</button></label>
                {/if}
              </div>
              {#if centerOverlayStyle !== "solid" && centerOverlayColorMode === "gradient" && centerOverlayGradientMode === "custom"}
                <label class="checkbox-label" style="font-size: 0.85rem; margin: 10px 0 8px;">
                  <input type="checkbox" bind:checked={centerOverlayUseFourthStop} disabled={!logoBase64} /> 4th Inner Stop
                </label>
                {#if centerOverlayUseFourthStop}
                  <div class="row color-row">
                    <label>Accent <input type="color" bind:value={centerOverlayColor4} disabled={!logoBase64}/> <input type="text" bind:value={centerOverlayColor4} class="hex-input" on:blur={(event) => onHexBlur('centerOverlayColor4', event)} disabled={!logoBase64}/> <button class="mini-color-btn" type="button" on:click={() => pickColorInto('centerOverlayColor4')} disabled={!logoBase64}>Pick</button></label>
                  </div>
                {/if}
              {/if}
            {/if}
          {/if}
          {#if centerOverlayMode === "match" && !enableFrame}
            <p class="sub-note">Turn on the outer overlay frame to mirror its look around the center photo.</p>
          {/if}
        </div>
      </fieldset>

      <div class="action-area">
        <button class="generate-btn" on:click={handleGenerateClick} disabled={loading}>
          {loading ? "PROCESSING..." : "GENERATE QR CODE"}
        </button>
        
        <div class="row split" style="margin-top: 10px;">
          <select bind:value={saveFormat} style="width: 30%; margin-bottom: 0;">
            <option value="png">PNG</option>
            <option value="jpg">JPG</option>
            <option value="svg">SVG</option>
          </select>
          <button class="save-btn" on:click={saveImage} disabled={!qrImagePng} style="width: 65%;">
            {isNativeMobileDevice() ? "💾 SAVE TO GALLERY" : "💾 SAVE IMAGE"}
          </button>
        </div>
        <button class="save-btn secondary-action" on:click={printCode} disabled={!qrImagePng}>
          PRINT CODE
        </button>
        <button class="save-btn secondary-action history-toggle-btn" type="button" on:click={() => showHistoryPanel = !showHistoryPanel}>
          HISTORY {generationHistory.length ? `(${generationHistory.length})` : ""}
        </button>
        {#if isNativeMobileDevice()}
          <p class="save-hint">Android saves straight to <strong>Gallery/Photos</strong> in <strong>Pictures/QR Studio Ultra</strong>.</p>
        {/if}
        {#if showSaveToast}
          <div 
            class={`save-toast ${saveToastTone}`}
            in:fly={{ y: 20, duration: 400, easing: backOut }}
            out:fade={{ duration: 300 }}
          >
            {#if saveToastTone === 'success'}
              <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" style="margin-right: 8px;"><polyline points="20 6 9 17 4 12"></polyline></svg>
            {:else if saveToastTone === 'error'}
              <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" style="margin-right: 8px;"><circle cx="12" cy="12" r="10"></circle><line x1="15" y1="9" x2="9" y2="15"></line><line x1="9" y1="9" x2="15" y2="15"></line></svg>
            {:else}
              <svg viewBox="0 0 24 24" width="18" height="18" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" style="margin-right: 8px;"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line></svg>
            {/if}
            <span>{mobileSaveMessage}</span>
          </div>
        {/if}
        {#if showMobileSaveActions && isNativeMobileDevice()}
          <div class="row split" style="margin-top: 10px;">
            <button class="generate-btn secondary-action" on:click={openSavedImage} style="width: 48%;">
              OPEN IN PHOTOS
            </button>
            <button class="generate-btn secondary-action" on:click={shareSavedImage} style="width: 48%;">
              SHARE
            </button>
          </div>
        {/if}
        {#if recentSaves.length}
          <div class="recent-saves">
            <div class="recent-saves-head">Recent Saves</div>
            {#each recentSaves as saveEntry, index}
              <div class={`recent-save-card ${index === 0 ? "latest" : ""}`}>
                <div>
                  <div class="recent-save-label">{saveEntry.label}</div>
                  <div class="recent-save-time">{saveEntry.timestamp}</div>
                </div>
                {#if index === 0 && isNativeMobileDevice()}
                  <div class="recent-save-actions">
                    <button class="mini-action" on:click={openSavedImage}>Open</button>
                    <button class="mini-action" on:click={shareSavedImage}>Share</button>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        {/if}
        {#if showHistoryPanel}
          <div class="history-panel" transition:fly={{ x: 20, duration: 300, easing: cubicOut }}>
            <div class="field-head">
              <div class="recent-saves-head">History</div>
              <button class="mini-action danger-mini" type="button" on:click={clearHistory} disabled={!generationHistory.length}>Clear</button>
            </div>
            {#if generationHistory.length}
              <div class="compact-list">
                {#each generationHistory as entry, i (entry.id)}
                  <div 
                    class="compact-card history-card"
                    in:fly={{ y: 10, duration: 300, delay: Math.min(i * 20, 400) }}
                  >
                    <div>
                      <strong>{entry.label}</strong>
                      <span>{entry.createdAt} · {entry.dataType} · {entry.score}%</span>
                    </div>
                    <div class="compact-actions">
                      <button class="mini-action" type="button" on:click={() => loadHistoryEntry(entry)}>Load</button>
                      <button class="mini-action" type="button" on:click={() => copyText(entry.payload)}>Copy</button>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="empty-history">Generated QR codes will appear here.</p>
            {/if}
          </div>
        {/if}
      </div>

      {#if qrImagePng}
        <div class="preview-area">
          <img src={qrImagePng} alt="QR Preview" />
          <div class={`scan-score-card preview-score ${currentScannability.className}`}>
            <div class="scan-score-head">
              <div>
                <div class="scan-score-label">Real-Time Scannability</div>
                <strong>{currentScannability.label}</strong>
              </div>
              <span>{currentScannability.score}%</span>
            </div>
            <div class="scan-score-bar">
              <span style={`width: ${currentScannability.score}%`}></span>
            </div>
          </div>
          <div class="quick-export-row">
            <button class="mini-action" type="button" on:click={() => quickSave("png")}>Download PNG</button>
            <button class="mini-action" type="button" on:click={() => quickSave("svg")}>Download SVG</button>
            <button class="mini-action" type="button" on:click={copySvg}>Copy SVG</button>
          </div>
          <div class="sub-panel print-title-panel">
            <p class="sub-label">Print Title</p>
            <input
              type="text"
              bind:value={printTitle}
              placeholder="Add a custom title for printed sheets..."
            />
            <p class="sub-note">This only changes the printed heading. It does not change the QR data.</p>
          </div>
        </div>
      {/if}

    </div>
  {/if}

  {#if qrImagePng}
    <section class="print-sheet" aria-hidden="true">
      <div class="print-card">
        <div class="print-brand">QR Studio Ultra</div>
        <h1>{printTitle.trim() || generatedLabel}</h1>
        <img src={qrImagePng} alt="Printable QR code" />
        <p class="print-type">{dataType} code</p>
        <p class="print-payload">{getPrintPayloadPreview()}</p>
        <p class="print-stamp">Generated {generatedAt}</p>
      </div>
    </section>
  {/if}
</main>

<style>
  .ai-magic-container {
    margin-bottom: 1rem;
    padding: 12px;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 12px;
    border: 1px solid rgba(255, 255, 255, 0.1);
  }
  .ai-prompt-textarea {
    width: 100%;
    margin-bottom: 0.8rem;
    font-size: 0.95rem;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid rgba(255, 255, 255, 0.15);
  }
  .ai-provider-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    margin-bottom: 10px;
  }
  .ai-provider-badge {
    display: inline-flex;
    align-items: center;
    border: 1px solid rgba(33, 212, 253, 0.26);
    border-radius: 999px;
    padding: 5px 10px;
    background: rgba(33, 212, 253, 0.1);
    color: #c8f6ff;
    font-size: 0.75rem;
    font-weight: 900;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }
  .ai-provider-note {
    color: #91a3b8;
    font-size: 0.75rem;
    text-align: right;
  }
  .ai-chips {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    margin-bottom: 1rem;
  }
  .ai-chip {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 20px;
    padding: 4px 12px;
    font-size: 0.8rem;
    color: #ddd;
    cursor: pointer;
    transition: all 0.2s;
  }
  .ai-chip:hover {
    background: rgba(255, 255, 255, 0.2);
    color: white;
  }
  .ai-force-center {
    margin: 0 0 12px 0;
    padding: 10px 12px;
    border-radius: 12px;
    background: rgba(33, 212, 253, 0.08);
    border: 1px solid rgba(33, 212, 253, 0.2);
    color: #dffaff;
  }
  .ai-center-prompt {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-bottom: 12px;
    padding: 10px 12px;
    border-radius: 12px;
    background: rgba(168, 85, 247, 0.1);
    border: 1px solid rgba(168, 85, 247, 0.22);
    color: #efe7ff;
    font-size: 0.82rem;
    line-height: 1.4;
  }
  .ai-center-prompt strong {
    color: #ffffff;
    font-size: 0.76rem;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }
  .ai-generate-btn {
    width: 100%;
    background: var(--accent-gradient);
    color: white;
    font-weight: bold;
    font-size: 1rem;
    border: none;
    padding: 12px;
    border-radius: 10px;
    cursor: pointer;
    transition: transform 0.2s, opacity 0.2s;
  }
  .ai-generate-btn:active {
    transform: scale(0.98);
  }
  .ai-generate-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  :global(body) { 
    background-color: #0F0F12; 
    color: #e0e0e0; 
    font-family: 'Segoe UI', Roboto, Helvetica, Arial, sans-serif; 
    margin: 0; 
    padding: 0;
    -webkit-tap-highlight-color: transparent; 
  }

  :global(body.scanning-active) { background-color: transparent !important; }

  .mobile-app { display: flex; flex-direction: column; max-width: 600px; margin: 0 auto; min-height: 100vh; position: relative; }

  /* CUSTOM MODAL STYLING */
  .custom-modal-overlay {
    position: fixed; top: 0; left: 0; width: 100%; height: 100vh;
    background: rgba(0, 0, 0, 0.7); backdrop-filter: blur(4px);
    display: flex; justify-content: center; align-items: center; z-index: 10000;
  }
  .custom-modal {
    background: linear-gradient(180deg, rgba(28, 28, 37, 0.98), rgba(18, 18, 24, 0.98));
    border: 1px solid rgba(255, 26, 146, 0.5); border-radius: 22px;
    padding: 24px; width: 90%; max-width: 400px; box-shadow: 0 22px 50px rgba(0, 0, 0, 0.38);
    text-align: center;
  }
  .custom-modal h3 { color: #ff7abb; margin-top: 0; font-size: 1.4rem; letter-spacing: -0.02em; }
  .custom-modal p { color: #d4d7df; font-size: 1rem; line-height: 1.45; margin-bottom: 16px; }
  .modal-kicker {
    display: inline-flex;
    margin-bottom: 12px;
    padding: 6px 12px;
    border-radius: 999px;
    background: rgba(255, 122, 187, 0.14);
    border: 1px solid rgba(255, 122, 187, 0.28);
    color: #ff9bca;
    font-size: 0.76rem;
    letter-spacing: 0.14em;
    text-transform: uppercase;
    font-weight: 800;
  }
  .warning-modal {
    border-color: rgba(255, 126, 95, 0.48);
    box-shadow: 0 24px 54px rgba(0, 0, 0, 0.42), 0 0 0 1px rgba(255, 126, 95, 0.12);
  }
  .warning-modal h3 { color: #ffd3bf; }
  .modal-actions { display: flex; gap: 10px; justify-content: space-between; }
  .modal-btn { flex: 1; padding: 12px; border-radius: 8px; font-weight: bold; font-size: 1rem; cursor: pointer; border: none; }
  .modal-btn:disabled { opacity: 0.55; cursor: not-allowed; }
  .modal-btn.outline { background: transparent; color: #aaa; border: 1px solid #555; }
  .modal-btn.danger { background: linear-gradient(135deg, #ff1a92 0%, #ff4b2b 100%); color: white; }
  .modal-btn.success-action { background: linear-gradient(135deg, #0faa73 0%, #52e6ad 100%); color: #03120c; opacity: 1; }
  .modal-btn.primary-modal { background: linear-gradient(135deg, #00c48c 0%, #21d4fd 100%); color: #061016; }
  .single-action { margin-top: 10px; }
  .decrypt-modal { border-color: rgba(33, 212, 253, 0.58); }
  .decrypt-modal h3 { color: #a9f1ff; }
  .crypto-meta {
    display: inline-flex;
    margin: -2px 0 12px;
    padding: 5px 10px;
    border-radius: 999px;
    background: rgba(33, 212, 253, 0.08);
    border: 1px solid rgba(33, 212, 253, 0.2);
    color: #a7eafd;
    font-size: 0.78rem;
    font-weight: 800;
  }
  .modal-passphrase { margin-bottom: 12px; text-align: left; }
  .modal-error {
    margin: 0 0 12px;
    padding: 10px 12px;
    border-radius: 8px;
    background: rgba(156, 42, 42, 0.26);
    border: 1px solid rgba(255, 108, 108, 0.32);
    color: #ffd3d3 !important;
    font-size: 0.9rem !important;
  }
  .modal-success {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    margin: 0 0 12px;
    padding: 10px 12px;
    border-radius: 8px;
    background: rgba(20, 132, 91, 0.24);
    border: 1px solid rgba(76, 225, 166, 0.34);
    color: #dfffee;
  }
  .modal-success-dismiss {
    position: absolute;
    top: 6px;
    right: 8px;
    width: 24px;
    height: 24px;
    border: 0;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.08);
    color: #c9ffe9;
    cursor: pointer;
    font-weight: 900;
    line-height: 1;
  }
  .modal-success span {
    display: grid;
    place-items: center;
    width: 22px;
    height: 22px;
    border-radius: 50%;
    background: #1fd196;
    color: #04140d;
    font-weight: 900;
  }
  .modal-decrypted {
    margin-bottom: 12px;
    text-align: left;
    max-height: 160px;
    overflow: auto;
  }
  .copy-plaintext-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
  }
  .copy-icon {
    position: relative;
    width: 14px;
    height: 16px;
    flex: 0 0 auto;
  }
  .copy-icon::before,
  .copy-icon::after {
    content: "";
    position: absolute;
    width: 10px;
    height: 12px;
    border: 1.5px solid currentColor;
    border-radius: 2px;
  }
  .copy-icon::before {
    left: 0;
    top: 3px;
    opacity: 0.5;
  }
  .copy-icon::after {
    right: 0;
    top: 0;
    background: rgba(255, 255, 255, 0.03);
  }

  /* CROP MODAL STYLING */
  .crop-modal { max-width: 500px; border-color: #21d4fd; box-shadow: 0 10px 30px rgba(33, 212, 253, 0.2); }
  .crop-modal h3 { color: #21d4fd; }
  .crop-container {
    position: relative; width: 100%; overflow: hidden; border-radius: 8px;
    cursor: grab; user-select: none; touch-action: none;
  }
  .crop-container:active { cursor: grabbing; }
  .crop-img { width: 100%; display: block; pointer-events: none; }
  .crop-box {
    position: absolute; border: 3px solid #21d4fd; 
    box-shadow: 0 0 0 9999px rgba(0, 0, 0, 0.55); 
    pointer-events: none; border-radius: 4px;
  }
  
  /* ZOOM SLIDER */
  .zoom-row { display: flex; align-items: center; gap: 10px; margin-top: 14px; padding: 0 4px; }
  .zoom-label { font-size: 0.9rem; color: #bbb; white-space: nowrap; }
  .zoom-value { font-size: 0.85rem; color: #21d4fd; font-weight: bold; min-width: 32px; text-align: right; }
  .zoom-slider { flex: 1; -webkit-appearance: none; appearance: none; height: 4px; border-radius: 2px; background: #3A3A45; outline: none; margin-bottom: 0 !important; padding: 0 !important; border: none !important; }
  .zoom-slider::-webkit-slider-thumb { -webkit-appearance: none; appearance: none; width: 20px; height: 20px; border-radius: 50%; background: linear-gradient(135deg, #21d4fd, #b721ff); cursor: pointer; }

  /* EXISTING STYLES */
  .lovely-header { background: #18181F; padding: 15px; border-bottom: 1px solid #2A2A33; position: sticky; top: 0; z-index: 100; }
  .logo-area { display: flex; align-items: center; justify-content: center; gap: 15px; }
  .logo-icon { width: 60px; height: 60px; }
  .logo-icon img { width: 100%; height: 100%; object-fit: contain; display: block; }
  .logo-text { display: flex; flex-direction: column; text-align: left; }
  .logo-text h1 { font-size: 1.6rem; margin: 0; color: #fff; letter-spacing: -0.5px; font-weight: 800; }
  .ultra { background: linear-gradient(135deg, #21d4fd 0%, #b721ff 100%); -webkit-background-clip: text; background-clip: text; -webkit-text-fill-color: transparent; font-weight: 900; }
  .pbess { font-size: 0.75rem; margin: 2px 0 0 0; color: #aaa; text-transform: uppercase; letter-spacing: 2px; font-weight: bold; }

  .scrolling-content { padding: 15px; padding-bottom: 40px; display: flex; flex-direction: column; gap: 15px; }

  .scanner-overlay { position: fixed; inset: 0; display: flex; flex-direction: column; justify-content: space-between; align-items: center; padding: 56px 20px 42px; background: radial-gradient(circle at center, rgba(0,0,0,0.16) 0, rgba(0,0,0,0.16) 18%, rgba(0,0,0,0.62) 65%, rgba(0,0,0,0.78) 100%); z-index: 9999; }
  .scanner-header h2 { color: #00FF00; font-size: 2rem; margin: 0 0 10px 0; text-shadow: 0 4px 6px rgba(0,0,0,0.9); text-align: center; }
  .scanner-header p { color: white; font-size: 1.1rem; font-weight: bold; text-shadow: 0 2px 4px rgba(0,0,0,0.9); text-align: center; }
  .scanner-stage { display: flex; flex-direction: column; align-items: center; gap: 18px; width: 100%; }
  .scanner-target { position: relative; width: min(78vw, 320px); aspect-ratio: 1 / 1; border: 2px solid rgba(255,255,255,0.22); border-radius: 24px; box-shadow: 0 0 0 999px rgba(0, 0, 0, 0.24); backdrop-filter: blur(1px); overflow: hidden; }
  .scanner-corner { position: absolute; width: 42px; height: 42px; border-color: #00ff88; border-style: solid; filter: drop-shadow(0 0 8px rgba(0, 255, 136, 0.75)); }
  .scanner-corner.top-left { top: -2px; left: -2px; border-width: 5px 0 0 5px; border-top-left-radius: 20px; }
  .scanner-corner.top-right { top: -2px; right: -2px; border-width: 5px 5px 0 0; border-top-right-radius: 20px; }
  .scanner-corner.bottom-left { bottom: -2px; left: -2px; border-width: 0 0 5px 5px; border-bottom-left-radius: 20px; }
  .scanner-corner.bottom-right { bottom: -2px; right: -2px; border-width: 0 5px 5px 0; border-bottom-right-radius: 20px; }
  .scanner-line { position: absolute; left: 7%; right: 7%; height: 3px; border-radius: 999px; background: linear-gradient(90deg, rgba(255,0,0,0), #ff3b30 20%, #ff6a6a 50%, #ff3b30 80%, rgba(255,0,0,0)); box-shadow: 0 0 18px rgba(255, 59, 48, 0.95); animation: scan-sweep 2.2s ease-in-out infinite; }
  .scanner-caption { color: #f5f7fa; font-size: 0.95rem; text-align: center; max-width: 290px; line-height: 1.4; text-shadow: 0 2px 8px rgba(0,0,0,0.8); }

  .result-panel { border-color: #00FF7F; background: #0d1a12; box-shadow: 0 0 15px rgba(0, 255, 127, 0.1); }
  .result-legend { color: #00FF7F; font-weight: 900; text-shadow: 0 0 5px rgba(0, 255, 127, 0.5); }
  .scan-meta { display: flex; justify-content: space-between; gap: 10px; margin-bottom: 10px; font-size: 0.78rem; text-transform: uppercase; letter-spacing: 0.08em; color: #8fd7ac; }
  .scanned-text-box { background: #000; color: #fff; padding: 12px; border-radius: 8px; font-family: monospace; word-break: break-all; border: 1px solid #224422; margin-bottom: 12px; }
  .result-actions { display: flex; gap: 8px; }
  .result-btn { flex: 1; padding: 12px 5px; border-radius: 8px; font-weight: bold; font-size: 0.85rem; border: none; cursor: pointer; }
  .primary-btn { background: #00FF7F; color: #000; }
  .secondary-btn { background: #333; color: #fff; }
  .cancel-btn { background: transparent; color: #aaa; border: 1px solid #555; }

  .cancel-scan-btn { background-color: #e53935; color: white; padding: 18px 40px; border: none; border-radius: 12px; font-size: 1.2rem; font-weight: 900; box-shadow: 0 6px 15px rgba(0,0,0,0.6); cursor: pointer; margin-bottom: 40px; }
  .activate-scan-btn { width: 100%; background: linear-gradient(135deg, #21d4fd 0%, #b721ff 100%); color: white; font-size: 1.1rem; font-weight: 900; padding: 18px; border: none; border-radius: 12px; box-shadow: 0 4px 15px rgba(183, 33, 255, 0.3); cursor: pointer; margin-bottom: 5px; }

  .panel { background-color: #18181F; border: 1px solid #2A2A33; border-radius: 12px; padding: 15px; margin: 0; }
  .pro-panel { border-color: #ff1a92; } 
  .blank-panel { border: none; padding: 0; background: transparent; }
  legend { font-weight: bold; font-size: 0.9rem; color: #aaa; padding: 0 8px; text-transform: uppercase; letter-spacing: 0.5px; }

  input[type="text"], input[type="password"], select, .text-area { width: 100%; background-color: #24242C; border: 1px solid #3A3A45; color: #fff; padding: 14px; border-radius: 8px; box-sizing: border-box; margin-bottom: 10px; font-size: 1rem; font-family: inherit; }
  .outline-select { background-color: #18181F; border: 1px dashed #444; color: #21d4fd; font-weight: bold; }
  .select-field {
    display: flex;
    flex: 1;
    min-width: 0;
    flex-direction: column;
    gap: 6px;
    color: #9aa7b7;
    font-size: 0.76rem;
    font-weight: 800;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }
  .select-field select {
    margin-bottom: 0;
    text-transform: none;
    letter-spacing: 0;
    font-size: 1rem;
    font-weight: 600;
  }
  .logo-safety-meter {
    position: relative;
    height: 10px;
    overflow: hidden;
    border-radius: 999px;
    background: #0b1016;
    border: 1px solid #283746;
    margin-top: 8px;
    margin-bottom: 4px;
  }
  .logo-safety-meter span {
    display: block;
    height: 100%;
    border-radius: inherit;
    transition: width 180ms ease, background-color 180ms ease;
  }
  .logo-safety-meter.safe span { background: #168a64; }
  .logo-safety-meter.warning span { background: #b8892f; }
  .logo-safety-meter.danger span { background: #a83c3c; }

  .text-area { resize: vertical; }
  input:disabled, select:disabled { opacity: 0.5; cursor: not-allowed; }

  .row { display: flex; align-items: center; gap: 10px; }
  .split { justify-content: space-between; }
  .full-width { width: 100%; }
  .mt-10 { margin-top: 10px; }
  .mb-10 { margin-bottom: 10px; }

  .color-row label { display: flex; flex-direction: column; font-size: 0.75rem; color: #bbb; gap: 6px; flex: 1; background: rgba(24, 24, 31, 0.88); border: 1px solid #2f3540; border-radius: 12px; padding: 10px; }
  .color-row label input[type="color"] { width: 100%; height: 44px; padding: 0; border: none; border-radius: 10px; cursor: pointer; background: transparent; }
  .color-row label input[type="color"]::-webkit-color-swatch-wrapper { padding: 0; }
  .color-row label input[type="color"]::-webkit-color-swatch { border: 1px solid #555; border-radius: 10px; }
  
  .hex-input { text-align: center; padding: 9px !important; font-size: 0.85rem !important; margin-bottom: 0 !important; background: #171c24 !important; border-color: #394453 !important; font-weight: 700; letter-spacing: 0.03em; }
  .range-value { display: inline-block; min-width: 52px; margin-left: 10px; color: #f7efe6; font-weight: 700; text-align: right; }
  .native-color-proxy { position: fixed; inset: auto auto -100px -100px; width: 1px; height: 1px; opacity: 0; pointer-events: none; }

  .sub-panel { background-color: #111115; border-radius: 8px; padding: 12px; margin-top: 15px; }
  .sub-label { font-size: 0.8rem; color: #888; margin: 0 0 10px 0; }
  .sub-note { margin: 10px 0 0 0; color: #93a4b8; font-size: 0.84rem; line-height: 1.4; }
  .intelligence-panel { border-color: rgba(156, 227, 194, 0.42); }
  .scan-score-card {
    padding: 13px;
    border-radius: 8px;
    background: #0d1117;
    border: 1px solid #263848;
  }
  .scan-score-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }
  .scan-score-head strong {
    display: block;
    color: #f7fbff;
    font-size: 1.05rem;
  }
  .scan-score-head > span {
    color: #ffffff;
    font-size: 1.25rem;
    font-weight: 900;
  }
  .scan-score-label {
    color: #8fa4b9;
    font-size: 0.74rem;
    font-weight: 900;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }
  .scan-score-bar {
    height: 9px;
    margin-top: 10px;
    overflow: hidden;
    border-radius: 999px;
    background: #06090d;
    border: 1px solid #22303d;
  }
  .scan-score-bar span {
    display: block;
    height: 100%;
    border-radius: inherit;
    transition: width 180ms ease;
  }
  .scan-score-card p {
    margin: 9px 0 0;
    color: #a8b8c8;
    font-size: 0.84rem;
    line-height: 1.35;
  }
  .scan-score-card.strong .scan-score-bar span { background: #16a66f; }
  .scan-score-card.good .scan-score-bar span { background: #9fb943; }
  .scan-score-card.warning .scan-score-bar span { background: #c8902f; }
  .scan-score-card.danger .scan-score-bar span { background: #bd4545; }
  .scan-score-card.empty .scan-score-bar span { background: transparent; }
  .preview-score {
    width: 100%;
    box-sizing: border-box;
  }
  .compact-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .compact-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 10px;
    border-radius: 8px;
    background: #0d1117;
    border: 1px solid #243446;
  }
  .compact-card div:first-child {
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 3px;
  }
  .compact-card strong {
    color: #f2f7fb;
    font-size: 0.9rem;
    word-break: break-word;
  }
  .compact-card span {
    color: #8ea0b7;
    font-size: 0.78rem;
  }
  .compact-actions {
    display: flex;
    gap: 6px;
    flex: 0 0 auto;
  }
  .danger-mini {
    background: rgba(95, 36, 44, 0.72) !important;
    color: #ffd7dc !important;
  }
  .mini-action:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }
  .batch-count {
    color: #9ce3c2;
    font-size: 0.78rem;
    font-weight: 900;
  }
  .batch-btn {
    width: 100%;
    background: linear-gradient(135deg, #155f7d 0%, #1ba784 100%);
    border-color: transparent;
  }
  .batch-template-btn {
    width: 100%;
    background: linear-gradient(135deg, #2d3c56 0%, #284f63 100%);
    border-color: #3a6074;
  }
  .batch-btn:disabled {
    opacity: 0.56;
    cursor: not-allowed;
  }
  .batch-template-btn:disabled {
    opacity: 0.56;
    cursor: not-allowed;
  }
  .batch-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 10px;
    margin-top: 12px;
  }
  .batch-card {
    min-width: 0;
    padding: 10px;
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 12px;
    background: #0b1016;
    display: flex;
    flex-direction: column;
    gap: 8px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    position: relative;
    overflow: hidden;
  }
  .batch-card:hover {
    border-color: rgba(56, 189, 248, 0.4);
    transform: translateY(-2px);
    box-shadow: 0 8px 20px rgba(0, 0, 0, 0.4);
    background: #151c24;
  }
  .batch-card img {
    width: 100%;
    aspect-ratio: 1 / 1;
    object-fit: contain;
    border-radius: 6px;
    background: #ffffff;
    transition: transform 0.3s ease;
  }
  .batch-card:hover img {
    transform: scale(1.02);
  }
  .batch-card-copy {
    display: flex;
    justify-content: space-between;
    gap: 8px;
    color: #dfefff;
    font-size: 0.82rem;
  }
  .batch-card-copy span {
    color: #9ce3c2;
    flex: 0 0 auto;
  }
  .history-panel {
    margin-top: 14px;
    padding: 12px;
    border-radius: 12px;
    background: rgba(17, 17, 21, 0.88);
    border: 1px solid #2b3a49;
  }
  .history-toggle-btn {
    border: 1px solid #36506b;
    background: linear-gradient(135deg, #1b3143 0%, #17202b 100%);
  }
  .history-card strong {
    max-width: 260px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .empty-history {
    margin: 10px 0 0;
    color: #8ea0b7;
    font-size: 0.9rem;
    text-align: center;
  }
  .wallet-row { align-items: stretch; }
  .wallet-toolbar { display: flex; flex-direction: column; gap: 10px; margin-top: 4px; }
  .wallet-btn { 
    border: 1px solid rgba(255, 255, 255, 0.1); 
    background: linear-gradient(135deg, #2a3a4a 0%, #1d2d3d 100%);
    color: #fff; 
    padding: 10px 14px; 
    border-radius: 12px; 
    cursor: pointer; 
    font-size: 0.94rem; 
    font-weight: 600;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }
  .wallet-btn:hover:not(:disabled) {
    background: linear-gradient(135deg, #354a5e 0%, #253748 100%);
    border-color: rgba(255, 255, 255, 0.2);
    transform: translateY(-1px);
    box-shadow: 0 6px 16px rgba(0, 0, 0, 0.3);
  }
  .wallet-btn:active:not(:disabled) {
    transform: translateY(1px);
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.4);
  }
  .progress-bar-bg {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.2);
  }
  .progress-bar-fill {
    height: 100%;
    background: linear-gradient(90deg, #1ba784 0%, #38bdf8 100%);
    transition: width 0.3s ease-out;
  }
  .mini-action {
    background: #2b3440;
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: #94a3b8;
    padding: 6px 12px;
    border-radius: 8px;
    cursor: pointer;
    font-size: 0.8rem;
    font-weight: 500;
    transition: all 0.2s ease;
  }
  .mini-action:hover:not(:disabled) {
    background: #3a4555;
    color: #f8fafc;
    border-color: rgba(255, 255, 255, 0.2);
  }
  .mini-action:active:not(:disabled) {
    transform: scale(0.96);
  }
  .wallet-save-btn { background: linear-gradient(135deg, #154b7d 0%, #1ba784 100%); border-color: transparent; }
  .wallet-delete-btn { background: #3c2024; border-color: #6a3038; }
  .wallet-payload-preview { background: #10141a; border: 1px dashed #36506b; color: #90e0ff; border-radius: 10px; padding: 12px; font-family: monospace; font-size: 0.8rem; word-break: break-all; }
  .wallet-library { display: flex; flex-direction: column; gap: 10px; }
  .wallet-card { display: flex; justify-content: space-between; align-items: center; gap: 12px; padding: 12px; background: #0d1117; border: 1px solid #243446; border-radius: 10px; }
  .wallet-card-copy { min-width: 0; display: flex; flex-direction: column; gap: 4px; }
  .wallet-meta { font-size: 0.8rem; color: #95a8bc; word-break: break-all; }
  .wallet-card-actions { display: flex; gap: 8px; }
  .crypto-panel { border-color: #21d4fd; }
  .crypto-tool { display: flex; flex-direction: column; gap: 10px; }
  .field-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 10px;
  }
  .field-head .sub-label { margin-bottom: 0; }
  .help-dot {
    width: 26px;
    height: 26px;
    border-radius: 50%;
    border: 1px solid #345365;
    background: #111a22;
    color: #8fe8ff;
    font-weight: 900;
    cursor: help;
    flex: 0 0 auto;
  }
  .passphrase-field {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 8px;
    align-items: stretch;
  }
  .passphrase-field input { margin-bottom: 0; }
  .reveal-btn {
    min-width: 68px;
    border: 1px solid #34485a;
    border-radius: 8px;
    background: #1a2530;
    color: #d9f5ff;
    font-weight: 800;
    cursor: pointer;
  }
  .strength-meter {
    position: relative;
    height: 22px;
    overflow: hidden;
    border-radius: 999px;
    background: #0b1016;
    border: 1px solid #283746;
  }
  .strength-meter span {
    display: block;
    height: 100%;
    border-radius: inherit;
    transition: width 180ms ease;
  }
  .strength-meter strong {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.74rem;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: #f5fbff;
  }
  .strength-meter.empty span { background: transparent; }
  .strength-meter.weak span { background: #a83c3c; }
  .strength-meter.good span { background: #b8892f; }
  .strength-meter.strong span { background: #168a64; }
  .algorithm-toggle {
    display: grid;
    grid-template-columns: 1fr 1fr auto;
    gap: 6px;
    padding: 4px;
    background: #0d1117;
    border: 1px solid #263848;
    border-radius: 8px;
  }
  .algorithm-toggle button {
    border: 0;
    border-radius: 6px;
    background: transparent;
    color: #9fb6c8;
    padding: 10px 8px;
    font-weight: 900;
    cursor: pointer;
    opacity: 0.62;
  }
  .algorithm-toggle button.active {
    background: linear-gradient(135deg, #176b87 0%, #21d4fd 100%);
    color: #ffffff;
    opacity: 1;
  }
  .algorithm-toggle .algorithm-help {
    width: 34px;
    height: auto;
    border-radius: 6px;
    padding: 0;
    opacity: 1;
  }
  .crypto-action-btn { width: 100%; background: linear-gradient(135deg, #116466 0%, #21d4fd 100%); border-color: transparent; }
  .crypto-action-btn:disabled { opacity: 0.55; cursor: not-allowed; }
  .encrypted-output,
  .decrypted-output {
    background: #080c11;
    border: 1px dashed #315e6d;
    color: #dffaff;
    border-radius: 10px;
    padding: 12px;
    font-family: ui-monospace, SFMono-Regular, Consolas, monospace;
    font-size: 0.82rem;
    line-height: 1.45;
    word-break: break-all;
    white-space: pre-wrap;
  }
  .forensic-date {
    margin-top: 10px;
    color: #9ce3c2;
    font-size: 0.82rem;
    font-weight: 800;
    letter-spacing: 0;
  }
  .forensic-date.verified {
    color: #9ce3c2;
  }
  .forensic-date.missing {
    color: #ffd166;
  }
  .decrypted-output {
    border-color: #4c725e;
    color: #effff4;
    word-break: break-word;
  }
  .mini-color-btn { margin-top: 8px; border: 1px solid #42566e; background: #202c39; color: #eef6ff; border-radius: 10px; padding: 8px 11px; font-size: 0.75rem; font-weight: 800; cursor: pointer; }
  .mini-color-btn:disabled { opacity: 0.45; cursor: not-allowed; }
  .studio-swatches { display: flex; flex-wrap: wrap; gap: 8px; margin-top: 12px; }
  .swatch-grid { display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; }
  .swatch { width: 32px; height: 32px; border-radius: 50%; border: 2px solid rgba(255,255,255,0.08); cursor: pointer; }
  
  .preset-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px; }
  .preset-container { display: flex; flex-direction: column; gap: 4px; }
  .preset-btn { width: 100%; border: 1px solid rgba(255,255,255,0.08); border-radius: 12px; padding: 13px 10px; color: white; font-size: 0.85rem; font-weight: 900; cursor: pointer; text-shadow: 1px 1px 3px rgba(0,0,0,0.8); }
  .reliability-bar { height: 4px; border-radius: 2px; width: 100%; opacity: 0.8; }
  .reliability-bar.green { background: #168a64; }
  .reliability-bar.yellow { background: #b8892f; }
  .reliability-bar.red { background: #a83c3c; }

  .checkbox-label { display: flex; align-items: center; gap: 12px; font-size: 1rem; margin-bottom: 12px; color: #eee; }
  .checkbox-label input[type="checkbox"] { width: 20px; height: 20px; }
  
  .upload-btn { width: 100%; background-color: #24242C; color: #fff; border: 1px dashed #555; padding: 16px; border-radius: 12px; font-weight: bold; cursor: pointer; }
  .upload-btn.has-logo { background: linear-gradient(135deg, #0f0f12 0%, #2b4570 100%); border-color: #21d4fd; }

  .action-area { margin-top: 10px; display: flex; flex-direction: column; gap: 12px; }
  
  .generate-btn { 
    width: 100%; 
    background: linear-gradient(135deg, #21d4fd 0%, #b721ff 100%); 
    color: white; 
    font-size: 1.1rem; 
    font-weight: 900; 
    padding: 20px; 
    border: none; 
    border-radius: 12px; 
    box-shadow: 0 4px 15px rgba(183, 33, 255, 0.3); 
    cursor: pointer; 
  }
  .generate-btn:disabled { background: #444; color: #888; box-shadow: none; cursor: not-allowed; }

  .save-btn { 
    width: 100%; 
    background: linear-gradient(135deg, #21d4fd 0%, #b721ff 100%); 
    color: white; 
    font-size: 1.1rem; 
    font-weight: 900; 
    padding: 14px; 
    border: none; 
    border-radius: 12px; 
    box-shadow: 0 4px 15px rgba(183, 33, 255, 0.3); 
    cursor: pointer; 
  }
  .save-btn:disabled { background: #444; color: #888; box-shadow: none; cursor: not-allowed; }

  .preview-area { 
    background-color: transparent; 
    border-radius: 12px; 
    padding: 20px; 
    display: flex; 
    flex-direction: column; 
    align-items: center; 
    gap: 20px; 
    margin-top: 10px;
    transition: all 0.4s ease;
    position: relative;
  }
  .preview-area.success-glow {
    box-shadow: 0 0 50px rgba(74, 222, 128, 0.3);
    transform: scale(1.02);
  }
  .preview-area img { max-width: 100%; border-radius: 12px; background-color: transparent; }
  .quick-export-row {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px;
  }
  .print-title-panel {
    width: 100%;
    margin-top: 0;
  }
  .print-title-panel input {
    margin-bottom: 0;
  }
  .print-sheet { display: none; }
  .print-card {
    width: 100%;
    max-width: 680px;
    margin: 0 auto;
    padding: 28px;
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 24px;
    background: #ffffff;
    color: #0f1720;
    text-align: center;
    box-sizing: border-box;
  }
  .print-brand {
    font-size: 0.82rem;
    letter-spacing: 0.2em;
    text-transform: uppercase;
    color: #526173;
    font-weight: 800;
  }
  .print-card h1 {
    margin: 10px 0 18px;
    font-size: 1.8rem;
    line-height: 1.15;
    color: #111827;
  }
  .print-card img {
    width: min(100%, 420px);
    display: block;
    margin: 0 auto 18px;
    border-radius: 18px;
  }
  .print-type,
  .print-payload,
  .print-stamp {
    margin: 0;
  }
  .print-type {
    font-size: 0.95rem;
    font-weight: 700;
    color: #233247;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }
  .print-payload {
    margin-top: 12px;
    color: #3c4858;
    line-height: 1.5;
    word-break: break-word;
  }
  .print-stamp {
    margin-top: 14px;
    font-size: 0.86rem;
    color: #6b7a8c;
  }
  .save-hint { margin: 0; text-align: center; font-size: 0.9rem; color: #b9c2cf; }
  .save-toast {
    position: fixed;
    bottom: 30px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 10000;
    padding: 14px 24px;
    border-radius: 14px;
    color: #FFFFFF;
    font-weight: 600;
    font-size: 0.95rem;
    display: flex;
    align-items: center;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.4), 0 0 0 1px rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(12px);
    pointer-events: none;
    max-width: 90vw;
    text-align: center;
    justify-content: center;
    gap: 10px;
  }
  .save-toast.success { 
    background: linear-gradient(135deg, rgba(28, 88, 52, 0.9), rgba(16, 48, 31, 0.95)); 
    border-left: 4px solid #4ade80;
    color: #ecfff0;
  }
  .save-toast.error { 
    background: linear-gradient(135deg, rgba(119, 31, 31, 0.9), rgba(64, 15, 15, 0.95)); 
    border-left: 4px solid #f87171;
    color: #fff0f0;
  }
  .save-toast.info { 
    background: linear-gradient(135deg, rgba(29, 54, 92, 0.9), rgba(16, 28, 51, 0.95)); 
    border-left: 4px solid #60a5fa;
    color: #eef5ff;
  }
  .secondary-action {
    background: linear-gradient(135deg, #2b3440 0%, #1c232b 100%);
    color: #f4f7fb;
    box-shadow: 0 10px 26px rgba(0, 0, 0, 0.22);
  }
  .recent-saves {
    margin-top: 14px;
    display: grid;
    gap: 8px;
  }
  .recent-saves-head {
    font-size: 0.78rem;
    letter-spacing: 0.18em;
    text-transform: uppercase;
    color: #8ea0b7;
    text-align: center;
  }
  .recent-save-card {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    padding: 12px 14px;
    border-radius: 16px;
    background: rgba(22, 28, 36, 0.88);
    border: 1px solid rgba(255, 255, 255, 0.06);
  }
  .recent-save-card.latest {
    border-color: rgba(110, 231, 183, 0.28);
    box-shadow: 0 0 0 1px rgba(110, 231, 183, 0.08), 0 12px 28px rgba(0, 0, 0, 0.2);
  }
  .recent-save-label {
    color: #f2f6fb;
    font-size: 0.95rem;
    font-weight: 600;
  }
  .recent-save-time {
    color: #8ea0b7;
    font-size: 0.82rem;
    margin-top: 2px;
  }
  .recent-save-actions {
    display: flex;
    gap: 8px;
  }
  .mini-action {
    border: 0;
    border-radius: 999px;
    padding: 8px 12px;
    background: rgba(255, 255, 255, 0.08);
    color: #f4f7fb;
    font-weight: 700;
    font-size: 0.8rem;
  }
  @keyframes save-toast-in {
    from {
      opacity: 0;
      transform: translateY(8px) scale(0.98);
    }
    to {
      opacity: 1;
      transform: translateY(0) scale(1);
    }
  }

  @keyframes scan-sweep {
    0% { top: 12%; }
    50% { top: calc(100% - 12% - 3px); }
    100% { top: 12%; }
  }

  .settings-nav-btn {
    background: rgba(255, 255, 255, 0.08);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: white;
    width: 36px;
    height: 36px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    transition: all 0.2s;
    font-size: 1.1rem;
  }
  .settings-nav-btn:hover {
    background: rgba(255, 255, 255, 0.15);
    transform: rotate(30deg);
  }

  @media (max-width: 520px) {
    .result-actions,
    .row.split,
    .wallet-card,
    .compact-card {
      flex-direction: column;
      align-items: stretch;
    }

    .batch-grid {
      grid-template-columns: 1fr;
    }

    .wallet-card-actions {
      width: 100%;
    }

    .wallet-card-actions .wallet-btn,
    .compact-actions .mini-action {
      flex: 1;
    }

    .compact-actions {
      width: 100%;
    }

    .scanner-target {
      width: min(82vw, 300px);
    }
  }

  @media print {
    :global(body) {
      background: #ffffff !important;
      color: #111827 !important;
      margin: 0 !important;
      padding: 0 !important;
      font-family: "Segoe UI", Roboto, Helvetica, Arial, sans-serif !important;
    }

    .mobile-app {
      max-width: none;
      min-height: auto;
      margin: 0;
      padding: 0;
      display: block;
    }

    .mobile-app > :not(.print-sheet) {
      display: none !important;
    }

    .print-sheet {
      display: block;
      padding: 0.5in;
    }

    .print-card {
      max-width: none;
      border: 0;
      border-radius: 0;
      box-shadow: none;
      padding: 0;
    }
  }
</style>

{:else if currentView === 'settings'}
  <Settings onBack={() => currentView = 'studio'} />
{/if}
