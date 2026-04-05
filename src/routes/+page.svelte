<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { scan, cancel, requestPermissions } from "@tauri-apps/plugin-barcode-scanner";

  // Data Types & Variables
  let dataType = "URL";
  let qrData = "https://yoursite";
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
  let ringColor = "#4A2B15";
  let frameTextColor = "#4A2B15";

  let logoBase64 = null;
  let logoName = "";
  let fileInput;

  // Render variables
  let qrImagePng = ""; 
  let qrImageJpg = ""; 
  let saveFormat = "png"; 
  let loading = false;
  
  // Scanner & Modal State
  let isScanning = false; 
  let scannedResult = ""; 
  let showDogTagWarning = false; 

  // --- NEW: Interactive Crop State ---
  let showCropModal = false;
  let cropRawSrc = "";
  let cropImgEl;
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

  function applySolid(c) { color1 = c; fillType = "Solid"; }
  function applyGradient(c1, c2) { color1 = c1; color2 = c2; fillType = "Linear"; }

  function triggerFileInput() { fileInput.click(); }
  
  // --- SMART AUTO-CROPPER LOGIC ---
  function handleLogoUpload(e) {
    const file = e.target.files[0];
    if (!file) return;
    logoName = file.name;
    const reader = new FileReader();
    reader.onload = (ev) => {
      cropRawSrc = ev.target.result;
      showCropModal = true;
    };
    reader.readAsDataURL(file);
  }

  function onCropImgLoad() {
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

  function onZoomChange(e) {
    cropZoom = parseFloat(e.target.value);
    updateCropSize();
  }

  function startDrag(e) {
    isDragging = true;
    const clientX = e.touches ? e.touches[0].clientX : e.clientX;
    const clientY = e.touches ? e.touches[0].clientY : e.clientY;
    dragStartX = clientX;
    dragStartY = clientY;
    cropStartX = cropX;
    cropStartY = cropY;
  }

  function onDrag(e) {
    if (!isDragging) return;
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

  // --- SAVE TO NATIVE GALLERY ---
  async function saveImage() {
    if (!qrImagePng) return;
    const b64Data = saveFormat === "jpg" ? qrImageJpg : qrImagePng;
    try {
      const msg = await invoke("save_to_device", { b64: b64Data, format: saveFormat });
      alert(msg);
    } catch (e) {
      alert("Failed to save: " + e);
    }
  }

  // --- NATIVE SCANNER FUNCTIONS ---
  async function startScan() {
    scannedResult = ""; 
    try {
      try {
        await requestPermissions();
      } catch (permErr) {
        alert("Camera Access Denied.\n\nPlease ensure you are using a mobile device and have granted camera permissions to use the scanner.");
        return;
      }

      isScanning = true;
      const result = await scan({ windowed: true });
      
      if (result && result.content) {
         scannedResult = result.content.trim();
      }
    } catch (e) {
      if (e !== "Canceled" && e !== "cancel") {
         alert("Scanner could not start: " + e);
      }
    } finally {
      isScanning = false;
    }
  }

  async function cancelScanner() {
    try { await cancel(); } catch (e) { console.error(e); }
    isScanning = false;
  }

  async function openLink(url) {
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
                  alert("Could not open link: " + err);
              }
          } catch (shareErr) {}
      }
  }

  async function copyText(text) {
      try {
          await navigator.clipboard.writeText(text);
          alert("Copied to clipboard!");
      } catch (err) {
          alert("Clipboard failed. Please copy manually.");
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
      alert(errorMsg);
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
    }

    try {
      let rustImageB64 = await invoke("generate_ultra_qr", {
        data: finalData,
        color1: color1,
        color2: color2,
        bgColor: bgColor,
        eyeOut: eyeOut,
        eyeIn: eyeIn,
        fillType: fillType,
        mainShape: mainShape,
        eyeShape: eyeShape,
        logoB64: logoBase64
      });

      const img = new Image();
      img.onload = () => {
        const canvas = document.createElement("canvas");
        const ctx = canvas.getContext("2d");

        if (enableFrame) {
          canvas.width = 800;
          canvas.height = 800;
          ctx.save();
          ctx.beginPath();
          ctx.arc(400, 400, 400, 0, Math.PI * 2);
          ctx.clip();
          ctx.fillStyle = bgColor;
          ctx.fillRect(0, 0, 800, 800);
          ctx.save();
          ctx.beginPath();
          ctx.arc(400, 400, 330, 0, Math.PI * 2);
          ctx.clip();
          ctx.drawImage(img, 120, 120, 560, 560);
          ctx.restore();
          ctx.beginPath();
          ctx.arc(400, 400, 350, 0, Math.PI * 2);
          ctx.strokeStyle = ringColor;
          ctx.lineWidth = 40;
          ctx.stroke();

          if (frameText) {
            ctx.font = "bold 44px 'Segoe UI', Arial, sans-serif";
            ctx.textAlign = "center";
            ctx.textBaseline = "middle";
            const textWidth = ctx.measureText(frameText.toUpperCase()).width;
            const badgeWidth = textWidth + 80;
            ctx.fillStyle = bgColor;
            ctx.fillRect(400 - (badgeWidth / 2), 700, badgeWidth, 90);
            ctx.fillStyle = frameTextColor;
            ctx.fillText(frameText.toUpperCase(), 400, 745);
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
            ctx.drawImage(img, 40, 40, 520, 520);
            ctx.restore();
          } else {
            ctx.fillStyle = bgColor;
            ctx.fillRect(0, 0, 600, 600);
            ctx.drawImage(img, 0, 0, 600, 600);
          }
        }

        qrImagePng = canvas.toDataURL("image/png");
        
        const jpgCanvas = document.createElement("canvas");
        jpgCanvas.width = canvas.width;
        jpgCanvas.height = canvas.height;
        const jctx = jpgCanvas.getContext("2d");
        jctx.fillStyle = "#FFFFFF"; 
        jctx.fillRect(0, 0, jpgCanvas.width, jpgCanvas.height);
        jctx.drawImage(canvas, 0, 0);
        
        qrImageJpg = jpgCanvas.toDataURL("image/jpeg", 1.0);
        loading = false;
      };
      img.src = rustImageB64;
    } catch (e) {
      console.error(e);
      alert("Error generating QR: " + e);
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
        <div class="custom-modal">
          <h3>⚠️ Legal Warning</h3>
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
          <div class="crop-container"
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
            <path d="M20,10 H10 V20" stroke-width="4" stroke="url(#LovelyGradient)" fill="none" rx="2"/>
            <path d="M80,10 H90 V20" stroke-width="4" stroke="url(#LovelyGradient)" fill="none" rx="2"/>
            <path d="M20,90 H10 V80" stroke-width="4" stroke="url(#LovelyGradient)" fill="none" rx="2"/>
            <path d="M80,90 H90 V80" stroke-width="4" stroke="url(#LovelyGradient)" fill="none" rx="2"/>
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
        {/if}
      </fieldset>

      <fieldset class="panel">
        <legend>2. Body & Colors</legend>
        <div class="row split mb-10">
          <select bind:value={bgShape} class="full-width outline-select">
            <option value="square">Outer Shape: Square Code</option>
            <option value="circle">Outer Shape: Round Sticker</option>
          </select>
        </div>

        <div class="row split">
          <select bind:value={mainShape}>
            <option value="square">Square Blocks</option>
            <option value="circle">Dots (Circles)</option>
            <option value="rounded">Rounded Blocks</option>
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
        </select>
        <div class="row split color-row mt-10">
          <label>Outer <input type="color" bind:value={eyeOut} /> <input type="text" bind:value={eyeOut} class="hex-input"/></label>
          <label>Inner <input type="color" bind:value={eyeIn} /> <input type="text" bind:value={eyeIn} class="hex-input"/></label>
        </div>
      </fieldset>

      <fieldset class="panel pro-panel">
        <legend>4. PRO Circular Frame</legend>
        <label class="checkbox-label">
          <input type="checkbox" bind:checked={enableFrame} /> Enable Ring Overlay
        </label>
        <input type="text" bind:value={frameText} placeholder="Scan Me" disabled={!enableFrame} />
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
            💾 SAVE TO GALLERY
          </button>
        </div>
      </div>

      {#if qrImagePng}
        <div class="preview-area">
          <img src={qrImagePng} alt="QR Preview" />
        </div>
      {/if}

    </div>
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
    background: #18181F; border: 1px solid #ff1a92; border-radius: 16px;
    padding: 24px; width: 90%; max-width: 400px; box-shadow: 0 10px 30px rgba(255, 26, 146, 0.2);
    text-align: center;
  }
  .custom-modal h3 { color: #ff1a92; margin-top: 0; font-size: 1.4rem; }
  .custom-modal p { color: #ccc; font-size: 1rem; line-height: 1.4; margin-bottom: 20px; }
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
  .zoom-slider { flex: 1; -webkit-appearance: none; height: 4px; border-radius: 2px; background: #3A3A45; outline: none; margin-bottom: 0 !important; padding: 0 !important; border: none !important; }
  .zoom-slider::-webkit-slider-thumb { -webkit-appearance: none; width: 20px; height: 20px; border-radius: 50%; background: linear-gradient(135deg, #21d4fd, #b721ff); cursor: pointer; }

  /* EXISTING STYLES */
  .lovely-header { background: #18181F; padding: 15px; border-bottom: 1px solid #2A2A33; position: sticky; top: 0; z-index: 100; }
  .logo-area { display: flex; align-items: center; justify-content: center; gap: 15px; }
  .logo-icon { width: 60px; height: 60px; }
  .logo-text { display: flex; flex-direction: column; text-align: left; }
  .logo-text h1 { font-size: 1.6rem; margin: 0; color: #fff; letter-spacing: -0.5px; font-weight: 800; }
  .ultra { background: linear-gradient(135deg, #21d4fd 0%, #b721ff 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; font-weight: 900; }
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
</style>