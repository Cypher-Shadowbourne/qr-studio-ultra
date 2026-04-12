<script lang="ts">
  import { tick } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { scan, cancel, requestPermissions } from "@tauri-apps/plugin-barcode-scanner";
  import { save } from "@tauri-apps/plugin-dialog";

  // Data Types & Variables
  let dataType = "URL";
  let qrData = "https://github.com/Cypher-Shadowbourne";
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

  // Main & Background
  let bgShape = "square";
  let mainShape = "square";
  let fillType = "Solid";
  let color1 = "#000000";
  let color2 = "#1a73e8";
  let bgColor = "#FFFFFF";

  // Eyes
  let eyeShape = "square";
  let eyeOut = "#000000";
  let eyeIn = "#000000";

  // PRO Settings & Logo
  let enableFrame = false;
  let frameText = "Scan Me";
  let ringStyle = "solid";
  let ringColor = "#4A2B15";
  let frameTextColor = "#4A2B15";
  let transparentTextBg = false;
  let transparentFrameBg = false;
  let matchTextStyle = false;
  let centerOverlayMode = "none";
  let centerOverlayStyle = "solid";
  let centerOverlayColor = "#4A2B15";
  const baseQrCanvasSize = 600;
  const baseLogoRatio = 0.22;

  let logoBase64: string | null = null;
  let logoName = "";
  let fileInput: HTMLInputElement;

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
  let generatedPayload = "";
  let generatedLabel = "QR Code";
  let printTitle = "";
  let generatedAt = "";
  
  // Scanner & Modal State
  let isScanning = false; 
  let scannedResult = ""; 
  let showDogTagWarning = false; 

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
    { name: "Sunset", c1: "#FF512F", c2: "#DD2476" },
    { name: "Ocean", c1: "#2193b0", c2: "#6dd5ed" },
    { name: "Forest", c1: "#11998e", c2: "#38ef7d" },
    { name: "Cyber", c1: "#f12711", c2: "#f5af19" },
    { name: "Coffee", c1: "#4A2B15", c2: "#2E1A0C" },
    { name: "Lava", c1: "#FF416C", c2: "#FF4B2B" },
    { name: "Midnight", c1: "#232526", c2: "#414345" },
    { name: "Berry", c1: "#8E2DE2", c2: "#4A00E0" }
  ];

  function applySolid(c: string) { color1 = c; fillType = "Solid"; }
  function applyGradient(c1: string, c2: string) { color1 = c1; color2 = c2; fillType = "Linear"; }

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

  // CRITICAL FIX: Synchronous, double-decode prevention
  function commitCrop() {
    const canvas = document.createElement("canvas");
    canvas.width = 200;
    canvas.height = 200;
    const ctx = canvas.getContext("2d");
    if (!ctx || !cropImgEl) return;
    
    // Reuse the already-decoded cropImgEl element directly!
    ctx.drawImage(cropImgEl, cropX, cropY, cropSize, cropSize, 0, 0, 200, 200);
    
    logoBase64 = canvas.toDataURL("image/png");
    showCropModal = false;
  }

  function cancelCrop() {
    showCropModal = false;
    logoName = "";
    cropRawSrc = "";
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
    requestAnimationFrame(() => {
      showSaveToast = true;
      saveToastTimer = setTimeout(() => {
        showSaveToast = false;
      }, 2600);
    });
  }

  function rememberRecentSave(label: string) {
    const timestamp = new Date().toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
    recentSaves = [{ label, timestamp }, ...recentSaves].slice(0, 3);
  }

  function getGeneratedLabel() {
    if (dataType === "DogTag" && petName.trim()) return `${petName.trim()} Dog Tag`;
    if (dataType === "WiFi" && wifiSsid.trim()) return `${wifiSsid.trim()} WiFi`;
    if (dataType === "vCard" && (vCardFirst.trim() || vCardLast.trim())) return `${vCardFirst.trim()} ${vCardLast.trim()}`.trim();
    if (dataType === "Email" && emailTo.trim()) return `Email for ${emailTo.trim()}`;
    if (dataType === "SMS" && smsPhone.trim()) return `SMS for ${smsPhone.trim()}`;
    if (dataType === "Phone" && phoneNum.trim()) return `Phone ${phoneNum.trim()}`;
    if (dataType === "Geo" && geoLat.trim() && geoLng.trim()) return `Location ${geoLat.trim()}, ${geoLng.trim()}`;
    if (dataType === "WhatsApp" && waPhone.trim()) return `WhatsApp ${waPhone.trim()}`;
    if (dataType === "Crypto" && cryptoType.trim()) return `${cryptoType.trim()} Wallet`;
    if (dataType === "Event" && eventTitle.trim()) return eventTitle.trim();
    if (dataType === "Social" && socialUser.trim()) return `${socialPlatform} ${socialUser.trim()}`;
    if (dataType === "LinkedIn" && linkedinUser.trim()) return `LinkedIn ${linkedinUser.trim()}`;
    if (dataType === "YouTube" && ytHandle.trim()) return `YouTube ${ytHandle.trim()}`;
    if (dataType === "TikTok" && tiktokUser.trim()) return `TikTok ${tiktokUser.trim()}`;
    if (dataType === "Zoom" && zoomMeetingId.trim()) return `Zoom ${zoomMeetingId.trim()}`;
    if (dataType === "URL" && qrData.trim()) return qrData.trim();
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

  // --- SAVE TO NATIVE GALLERY ---
  async function saveImage() {
    if (!qrImagePng) return;
    const b64Data = saveFormat === "jpg" ? qrImageJpg : qrImagePng;
    mobileSaveMessage = "";
    showMobileSaveActions = false;
    
    try {
      const isMobile = isNativeMobileDevice();

      if (isMobile) {
        const result = await invoke<{ message: string }>("save_to_device", { b64: b64Data, format: saveFormat });
        showSaveToastMessage(result.message, "success");
        showMobileSaveActions = true;
        rememberRecentSave(`Saved ${saveFormat.toUpperCase()} to Gallery`);
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
          const msg = await invoke("save_to_path", { b64: b64Data, path: filePath });
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
    try {
      try {
        await requestPermissions();
      } catch (permErr) {
        showSaveToastMessage("Camera access denied. Please grant camera permission on your mobile device to use the scanner.", "error");
        return;
      }

      isScanning = true;
      const result = await scan({ windowed: true });
      
      if (result && result.content) {
         scannedResult = result.content.trim();
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
          showSaveToastMessage("Clipboard failed. Please copy manually.", "error");
      }
  }

  // --- VALIDATION ---
  function validateInputs() {
    if (dataType === "URL" && !qrData.trim()) return "Please enter a valid URL or text.";
    if (dataType === "WiFi" && !wifiSsid.trim()) return "Please enter the WiFi Network Name (SSID).";
    if (dataType === "vCard" && (!vCardFirst.trim() || !vCardPhone.trim())) return "Please enter at least a First Name and Phone Number for the contact.";
    if (dataType === "Email" && !emailTo.trim()) return "Please enter a destination Email Address.";
    if (dataType === "SMS" && !smsPhone.trim()) return "Please enter a destination Phone Number.";
    if (dataType === "Phone" && !phoneNum.trim()) return "Please enter a Phone Number.";
    
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

  async function runGeneration() {
    showDogTagWarning = false;
    loading = true;
    
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
      finalData = `${cryptoType}:${cryptoAddr}`;
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

    try {
      const rustImageB64 = await invoke<string>("generate_ultra_qr", {
        options: {
          data: finalData,
          color1: color1,
          color2: color2,
          bgColor: bgColor,
          eyeOut: eyeOut,
          eyeIn: eyeIn,
          fillType: fillType,
          mainShape: mainShape,
          bgShape: bgShape,
          eyeShape: eyeShape,
          logoB64: logoBase64
        }
      });

      const img = new Image();
      img.onload = () => {
        const canvas = document.createElement("canvas");
        const ctx = canvas.getContext("2d");
        if (!ctx) return;

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
            c.lineTo(right, cy - side);
            c.lineTo(right, cy + side);
            c.lineTo(cx + side, bottom);
            c.lineTo(cx - side, bottom);
            c.lineTo(left, cy + side);
            c.lineTo(left, cy - side);
            c.closePath();
          } else {
            c.rect(cx - sz / 2, cy - sz / 2, sz, sz);
          }
        };

        const drawRingDecoration = (
          c: CanvasRenderingContext2D,
          style: string,
          shape: string,
          cx: number,
          cy: number,
          size: number,
          color: string
        ) => {
          if (style === "none") return;

          c.save();
          c.strokeStyle = color;
          c.fillStyle = color;

          if (style === "gradient") {
            const grad = c.createLinearGradient(cx - size / 2, cy - size / 2, cx + size / 2, cy + size / 2);
            grad.addColorStop(0, color1);
            grad.addColorStop(1, color2);
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

        const drawCenterOverlay = (c: CanvasRenderingContext2D, size: number) => {
          if (!logoBase64 || centerOverlayMode === "none") return;
          if (centerOverlayMode === "match" && !enableFrame) return;
          const overlayStyle = centerOverlayMode === "match" && enableFrame ? ringStyle : centerOverlayStyle;
          const overlayColor = centerOverlayMode === "match" && enableFrame ? ringColor : centerOverlayColor;
          if (overlayStyle === "none") return;
          drawRingDecoration(c, overlayStyle, bgShape, canvas.width / 2, canvas.height / 2, size, overlayColor);
        };

        const getCenterOverlaySize = (renderedQrSize: number) => {
          const logoRenderedSize = renderedQrSize * baseLogoRatio;
          return Math.max(24, logoRenderedSize - Math.max(2, logoRenderedSize * 0.02));
        };

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
          
          // Draw the Ring Frame
          drawRingDecoration(ctx, ringStyle, bgShape, 400, 400, 700, ringColor);

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
          drawCenterOverlay(ctx, getCenterOverlaySize(qrSize));
          ctx.restore();

          if (frameText) {
            ctx.save();
            ctx.font = "bold 44px 'Segoe UI', Arial, sans-serif";
            ctx.textAlign = "center";
            ctx.textBaseline = "middle";
            const textWidth = ctx.measureText(frameText.toUpperCase()).width;
            const badgeWidth = textWidth + 80;
            const badgeHeight = 70;
            
            // Adjust badge position for different shapes
            let badgeY = 675;
            if (bgShape === "diamond") badgeY = 630;
            else if (bgShape === "octagon") badgeY = 660;

            if (!transparentTextBg) {
              ctx.fillStyle = bgColor;
              if (matchTextStyle && ringStyle === "neon") {
                ctx.shadowColor = ringColor;
                ctx.shadowBlur = 20;
              }

              // Draw badge background
              const bx = 400 - badgeWidth / 2;
              const by = badgeY - badgeHeight / 2;
              
              if (matchTextStyle && (ringStyle === "rounded" || bgShape === "rounded")) {
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

              // Optional badge border if matching
              if (matchTextStyle) {
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
            
            // Text color logic
            if (matchTextStyle) {
              if (ringStyle === "gradient") {
                const grad = ctx.createLinearGradient(400 - textWidth/2, 0, 400 + textWidth/2, 0);
                grad.addColorStop(0, color1);
                grad.addColorStop(1, color2);
                ctx.fillStyle = grad;
              } else if (ringStyle === "neon") {
                ctx.fillStyle = "#FFFFFF";
                ctx.shadowColor = ringColor;
                ctx.shadowBlur = 10;
              } else if (ringStyle === "diamond") {
                ctx.fillStyle = ringColor;
                // Add tiny diamonds next to text?
                ctx.save();
                ctx.translate(400 - textWidth/2 - 25, badgeY);
                ctx.rotate(Math.PI/4); ctx.fillRect(-8, -8, 16, 16);
                ctx.restore();
                ctx.save();
                ctx.translate(400 + textWidth/2 + 25, badgeY);
                ctx.rotate(Math.PI/4); ctx.fillRect(-8, -8, 16, 16);
                ctx.restore();
              } else {
                ctx.fillStyle = frameTextColor;
              }
            } else {
              ctx.fillStyle = frameTextColor;
            }
            
            ctx.fillText(frameText.toUpperCase(), 400, badgeY);
            ctx.restore();
          }
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
            // QR at 400x400 prevents corner clipping on round background
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
            // Draw larger QR so corners are visibly rounded by clipping
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
            // Diamond needs much more padding to fit square QR
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
            // Increased padding to ensure eyes are safe but shape is distinct
            ctx.drawImage(img, 60, 60, 480, 480);
            ctx.restore();
          } else {
            ctx.fillStyle = bgColor;
            ctx.fillRect(0, 0, 600, 600);
            ctx.drawImage(img, 0, 0, 600, 600);
          }
          drawCenterOverlay(ctx, getCenterOverlaySize(baseQrCanvasSize));
        }

        qrImagePng = canvas.toDataURL("image/png");
        
        const jpgCanvas = document.createElement("canvas");
        jpgCanvas.width = canvas.width;
        jpgCanvas.height = canvas.height;
        const jctx = jpgCanvas.getContext("2d");
        if (!jctx) return;
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
        loading = false;
      };
      img.src = rustImageB64;
    } catch (e: any) {
      console.error(e);
      showSaveToastMessage("Error generating QR: " + (e.message || e), "error");
      loading = false;
    }
  }

  $: if (typeof document !== 'undefined') {
    if (isScanning) {
      document.body.classList.add('scanning-active');
    } else {
      document.body.classList.remove('scanning-active');
    }
  }
</script>

<main class="mobile-app">
  {#if isScanning}
    <div class="scanner-overlay">
      <div class="scanner-header">
        <h2>SCANNING...</h2>
        <p>Point camera at a QR Code</p>
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
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100%" height="100%">
            <defs>
              <linearGradient id="LovelyGradient" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" stop-color="#21d4fd" />
                <stop offset="35%" stop-color="#b721ff" />
                <stop offset="65%" stop-color="#ff1a92" />
                <stop offset="100%" stop-color="#ffd700" />
              </linearGradient>
            </defs>
            <path d="M22,10 H12 A2,2 0 0 0 10,12 V22" stroke-width="4" stroke="url(#LovelyGradient)" fill="none" />
            <path d="M78,10 H88 A2,2 0 0 1 90,12 V22" stroke-width="4" stroke="url(#LovelyGradient)" fill="none" />
            <path d="M22,90 H12 A2,2 0 0 1 10,88 V78" stroke-width="4" stroke="url(#LovelyGradient)" fill="none" />
            <path d="M78,90 H88 A2,2 0 0 0 90,88 V78" stroke-width="4" stroke="url(#LovelyGradient)" fill="none" />
            <g transform="translate(20, 20) scale(0.6)" fill="url(#LovelyGradient)" >
              <path d="M50,0 C65.5,0,78.2,12.7,78.2,28.2 C78.2,38.5,72.6,47.7,64.1,52.8 L64.1,72.3 C64.1,76.5,60.7,80,56.5,80 L43.5,80 C39.3,80,35.9,76.5,35.9,72.3 L35.9,52.8 C27.4,47.7,21.8,38.5,21.8,28.2 C21.8,12.7,34.5,0,50,0 M50,11.3 C40.7,11.3,33.1,18.9,33.1,28.2 C33.1,34.1,36.2,39.3,40.7,42.4 C42.1,43.3,42.9,44.9,42.9,46.5 L42.9,68.7 L57.1,68.7 L57.1,46.5 C57.1,44.9,57.9,43.3,59.3,42.4 C63.8,39.3,66.9,34.1,66.9,28.2 C66.9,18.9,59.3,11.3,50,11.3"/>
              <circle cx="50" cy="28.2" r="14.1" fill="none" stroke-width="4" stroke="url(#LovelyGradient)"/>
              <path d="M50,28.2 L35.9,28.2" stroke-width="4" stroke="url(#LovelyGradient)"/>
              <path d="M50,28.2 L50,42.3" stroke-width="4" stroke="url(#LovelyGradient)"/>
              <g transform="translate(68, 68) scale(0.18)" fill="#00FF7F">
                <path d="M50,0 C65,0,80,15,80,30 C80,45,65,60,50,60 C35,60,20,45,20,30 C20,15,35,0,50,0"/>
                <path d="M50,40 C65,40,80,55,80,70 C80,85,65,100,50,100 C35,100,20,85,20,70 C20,55,35,40,50,40"/>
                <path d="M0,50 C0,35,15,20,30,20 C45,20,60,35,60,50 C60,65,45,80,30,80 C15,80,0,65,0,50"/>
                <path d="M45,80 V100" stroke-width="6" stroke="#00FF7F"/>
              </g>
            </g>
          </svg>
        </div>
        <div class="logo-text">
          <h1>QR STUDIO <span class="ultra">ULTRA</span></h1>
          <p class="pbess">BY CYPHER SHADOWBOURNE</p>
        </div>
      </div>
    </header>

    <div class="scrolling-content">
      <button class="activate-scan-btn" on:click={startScan}>📸 SCAN A QR CODE</button>

      {#if scannedResult}
        <fieldset class="panel result-panel">
          <legend class="result-legend">Scanned Result</legend>
          <div class="scanned-text-box">{scannedResult}</div>
          <div class="result-actions">
            <button class="result-btn primary-btn" on:click={() => openLink(scannedResult)}>OPEN LINK</button>
            <button class="result-btn secondary-btn" on:click={() => copyText(scannedResult)}>COPY TEXT</button>
            <button class="result-btn cancel-btn" on:click={() => scannedResult = ''}>CLEAR</button>
          </div>
        </fieldset>
      {/if}

      <fieldset class="panel">
        <legend>1. Content Data</legend>
        <select bind:value={dataType}>
          <option value="URL">Standard Link / Text</option>
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
          <input type="text" bind:value={qrData} placeholder="Paste Link or Text Here..." />
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
          <div class="row split">
            <select bind:value={cryptoType} class="outline-select">
              <option value="bitcoin">Bitcoin (BTC)</option>
              <option value="ethereum">Ethereum (ETH)</option>
              <option value="litecoin">Litecoin (LTC)</option>
            </select>
            <input type="text" bind:value={cryptoAddr} placeholder="Wallet Address" />
          </div>
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

      <fieldset class="panel">
        <legend>2. Body & Colors</legend>
        <div class="row split mb-10">
          <select bind:value={bgShape} class="full-width outline-select">
            <option value="square">Outer: Square</option>
            <option value="circle">Outer: Round Sticker</option>
            <option value="rounded">Outer: Rounded Square</option>
            <option value="diamond">Outer: Diamond</option>
            <option value="octagon">Outer: Octagon</option>
          </select>
        </div>

        <div class="row split">
          <select bind:value={mainShape}>
            <option value="square">Square Blocks</option>
            <option value="circle">Dots (Circles)</option>
            <option value="rounded">Rounded Blocks</option>
            <option value="diamond">Diamond Blocks</option>
          </select>
          <select bind:value={fillType}>
            <option value="Solid">Solid Color</option>
            <option value="Linear">Gradient</option>
          </select>
        </div>
        
        <div class="row split color-row mt-10">
          <label>Main <input type="color" bind:value={color1} /> <input type="text" bind:value={color1} class="hex-input"/></label>
          <label>End <input type="color" bind:value={color2} /> <input type="text" bind:value={color2} class="hex-input"/></label>
        </div>
        
        <div class="row color-row mt-10">
          <label>BG <input type="color" bind:value={bgColor} /> <input type="text" bind:value={bgColor} class="hex-input"/></label>
        </div>

        <div class="sub-panel">
          <p class="sub-label">Solid Colors</p>
          <div class="swatch-grid">
            {#each solidColors as color}
              <button class="swatch" style="background: {color};" on:click={() => applySolid(color)} aria-label={color}></button>
            {/each}
          </div>
        </div>

        <div class="sub-panel">
          <p class="sub-label">Gradients</p>
          <div class="preset-grid">
            {#each gradientPresets as preset}
              <button class="preset-btn" style="background: linear-gradient(to right, {preset.c1}, {preset.c2});" on:click={() => applyGradient(preset.c1, preset.c2)}>
                {preset.name}
              </button>
            {/each}
          </div>
        </div>
      </fieldset>

      <fieldset class="panel">
        <legend>3. Finder Patterns (Eyes)</legend>
        <select bind:value={eyeShape} class="full-width">
          <option value="square">Square Eyes</option>
          <option value="circle">Circular Eyes</option>
          <option value="diamond">Diamond Eyes</option>
          <option value="rounded">Rounded Eyes</option>
        </select>
        <div class="row split color-row mt-10">
          <label>Outer <input type="color" bind:value={eyeOut} /> <input type="text" bind:value={eyeOut} class="hex-input"/></label>
          <label>Inner <input type="color" bind:value={eyeIn} /> <input type="text" bind:value={eyeIn} class="hex-input"/></label>
        </div>
      </fieldset>

      <fieldset class="panel pro-panel">
        <legend>4. PRO Overlay Frame</legend>
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={enableFrame} /> Enable Ring Overlay
        </label>
        <div class="row split">
          <label class="checkbox-label" style="font-size: 0.85rem; margin-bottom: 12px;">
            <input type="checkbox" bind:checked={transparentFrameBg} disabled={!enableFrame} /> Transparent Frame BG
          </label>
          <label class="checkbox-label" style="font-size: 0.85rem; margin-bottom: 12px;">
            <input type="checkbox" bind:checked={transparentTextBg} disabled={!enableFrame} /> Transparent Text BG
          </label>
        </div>
        <div class="row split">
          <label class="checkbox-label" style="font-size: 0.85rem; margin-bottom: 12px;">
            <input type="checkbox" bind:checked={matchTextStyle} disabled={!enableFrame} /> Match Text to Style
          </label>
        </div>
        <input type="text" bind:value={frameText} placeholder="Scan Me" disabled={!enableFrame} />
        <div class="row split mt-10">
          <select bind:value={ringStyle} disabled={!enableFrame} class="full-width">
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
        </div>
        <div class="row split color-row mt-10">
          <label>Ring <input type="color" bind:value={ringColor} disabled={!enableFrame}/> <input type="text" bind:value={ringColor} class="hex-input" disabled={!enableFrame}/></label>
          <label>Text <input type="color" bind:value={frameTextColor} disabled={!enableFrame}/> <input type="text" bind:value={frameTextColor} class="hex-input" disabled={!enableFrame}/></label>
        </div>
      </fieldset>

      <fieldset class="panel blank-panel">
        <input type="file" accept="image/png, image/jpeg" bind:this={fileInput} on:change={handleLogoUpload} style="display: none;" />
        <button class="upload-btn" on:click={triggerFileInput} class:has-logo={logoName !== ""}>
          {logoName !== "" ? `Logo Loaded: ${logoName}` : "Upload Center Logo"}
        </button>
        <div class="sub-panel mt-10">
          <p class="sub-label">Center Photo Overlay</p>
          <select bind:value={centerOverlayMode} class="full-width" disabled={!logoBase64}>
            <option value="none">No Inner Overlay</option>
            <option value="match" disabled={!enableFrame}>Match Outer Overlay</option>
            <option value="custom">Custom Inner Overlay</option>
          </select>
          {#if centerOverlayMode === "custom"}
            <div class="row split mt-10">
              <select bind:value={centerOverlayStyle} class="full-width" disabled={!logoBase64}>
                <option value="solid">Solid Ring</option>
                <option value="double">Double Ring</option>
                <option value="dotted">Dotted Ring</option>
                <option value="dashed">Dashed Ring</option>
                <option value="rounded">Rounded Ring</option>
                <option value="diamond">Diamond Ring</option>
                <option value="gradient">Gradient Ring</option>
                <option value="neon">Neon Glow Ring</option>
              </select>
            </div>
            <div class="row split color-row mt-10">
              <label>Inner Ring <input type="color" bind:value={centerOverlayColor} disabled={!logoBase64}/> <input type="text" bind:value={centerOverlayColor} class="hex-input" disabled={!logoBase64}/></label>
            </div>
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
          </select>
          <button class="save-btn" on:click={saveImage} disabled={!qrImagePng} style="width: 65%;">
            {isNativeMobileDevice() ? "💾 SAVE TO GALLERY" : "💾 SAVE IMAGE"}
          </button>
        </div>
        <button class="save-btn secondary-action" on:click={printCode} disabled={!qrImagePng}>
          PRINT CODE
        </button>
        {#if isNativeMobileDevice()}
          <p class="save-hint">Android saves straight to <strong>Gallery/Photos</strong> in <strong>Pictures/QR Studio Ultra</strong>.</p>
        {/if}
        {#if showSaveToast}
          <div class={`save-toast ${saveToastTone}`}>
            <span class="save-toast-dot"></span>
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
      </div>

      {#if qrImagePng}
        <div class="preview-area">
          <img src={qrImagePng} alt="QR Preview" />
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
  .modal-btn.outline { background: transparent; color: #aaa; border: 1px solid #555; }
  .modal-btn.danger { background: linear-gradient(135deg, #ff1a92 0%, #ff4b2b 100%); color: white; }

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
  .logo-text { display: flex; flex-direction: column; text-align: left; }
  .logo-text h1 { font-size: 1.6rem; margin: 0; color: #fff; letter-spacing: -0.5px; font-weight: 800; }
  .ultra { background: linear-gradient(135deg, #21d4fd 0%, #b721ff 100%); -webkit-background-clip: text; background-clip: text; -webkit-text-fill-color: transparent; font-weight: 900; }
  .pbess { font-size: 0.75rem; margin: 2px 0 0 0; color: #aaa; text-transform: uppercase; letter-spacing: 2px; font-weight: bold; }

  .scrolling-content { padding: 15px; padding-bottom: 40px; display: flex; flex-direction: column; gap: 15px; }

  .scanner-overlay { position: fixed; top: 0; left: 0; width: 100%; height: 100vh; display: flex; flex-direction: column; justify-content: space-between; align-items: center; padding: 80px 20px; background: rgba(0, 0, 0, 0.4); z-index: 9999; }
  .scanner-header h2 { color: #00FF00; font-size: 2rem; margin: 0 0 10px 0; text-shadow: 0 4px 6px rgba(0,0,0,0.9); text-align: center; }
  .scanner-header p { color: white; font-size: 1.1rem; font-weight: bold; text-shadow: 0 2px 4px rgba(0,0,0,0.9); text-align: center; }

  .result-panel { border-color: #00FF7F; background: #0d1a12; box-shadow: 0 0 15px rgba(0, 255, 127, 0.1); }
  .result-legend { color: #00FF7F; font-weight: 900; text-shadow: 0 0 5px rgba(0, 255, 127, 0.5); }
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
  .text-area { resize: vertical; }
  input:disabled, select:disabled { opacity: 0.5; cursor: not-allowed; }

  .row { display: flex; align-items: center; gap: 10px; }
  .split { justify-content: space-between; }
  .full-width { width: 100%; }
  .mt-10 { margin-top: 10px; }
  .mb-10 { margin-bottom: 10px; }

  .color-row label { display: flex; flex-direction: column; font-size: 0.75rem; color: #bbb; gap: 4px; flex: 1; }
  .color-row label input[type="color"] { width: 100%; height: 40px; padding: 0; border: none; border-radius: 8px; cursor: pointer; background: transparent; }
  .color-row label input[type="color"]::-webkit-color-swatch-wrapper { padding: 0; }
  .color-row label input[type="color"]::-webkit-color-swatch { border: 1px solid #555; border-radius: 8px; }
  
  .hex-input { text-align: center; padding: 8px !important; font-size: 0.85rem !important; margin-bottom: 0 !important; }

  .sub-panel { background-color: #111115; border-radius: 8px; padding: 12px; margin-top: 15px; }
  .sub-label { font-size: 0.8rem; color: #888; margin: 0 0 10px 0; }
  .sub-note { margin: 10px 0 0 0; color: #93a4b8; font-size: 0.84rem; line-height: 1.4; }
  .swatch-grid { display: flex; flex-wrap: wrap; gap: 8px; justify-content: center; }
  .swatch { width: 32px; height: 32px; border-radius: 50%; border: 2px solid #222; cursor: pointer; }
  
  .preset-grid { display: grid; grid-template-columns: repeat(2, 1fr); gap: 10px; }
  .preset-btn { border: none; border-radius: 8px; padding: 12px 0; color: white; font-size: 0.85rem; font-weight: bold; cursor: pointer; text-shadow: 1px 1px 3px rgba(0,0,0,0.8); }

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

  .preview-area { background-color: transparent; border-radius: 12px; padding: 20px; display: flex; flex-direction: column; align-items: center; gap: 20px; margin-top: 10px; }
  .preview-area img { max-width: 100%; border-radius: 12px; background-color: transparent; }
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
    margin-top: 12px;
    padding: 12px 14px;
    border-radius: 16px;
    display: flex;
    align-items: center;
    gap: 10px;
    font-size: 0.95rem;
    animation: save-toast-in 280ms ease-out;
    box-shadow: 0 14px 32px rgba(0, 0, 0, 0.24);
  }
  .save-toast.success { background: linear-gradient(135deg, rgba(28, 88, 52, 0.95), rgba(16, 48, 31, 0.96)); color: #ecfff0; }
  .save-toast.error { background: linear-gradient(135deg, rgba(119, 31, 31, 0.96), rgba(64, 15, 15, 0.98)); color: #fff0f0; }
  .save-toast.info { background: linear-gradient(135deg, rgba(29, 54, 92, 0.96), rgba(16, 28, 51, 0.98)); color: #eef5ff; }
  .save-toast-dot {
    width: 10px;
    height: 10px;
    border-radius: 999px;
    background: currentColor;
    box-shadow: 0 0 18px currentColor;
    flex: 0 0 auto;
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
