<script lang="ts">
    import { onMount } from "svelte";

    interface PrinterInfo {
        name: string;
        is_default: boolean;
        status: string;
    }

    interface PrinterCheckResult {
        connected: boolean;
        printers: PrinterInfo[];
        default_printer: string | null;
        error: string | null;
    }

    interface TestPrintResult {
        success: boolean;
        message: string;
    }

    let printerResult: PrinterCheckResult = $state({
        connected: false,
        printers: [],
        default_printer: null,
        error: null,
    });

    let isChecking = $state(false);
    let isPrinting = $state(false);
    let testPrintMessage = $state("");
    let showTestResult = $state(false);
    let isTauriAvailable = $state(false);

    // Printer Network Config
    let printerIp = $state("");
    let printerPort = $state("9100");
    let isNetworkVerified = $state(false);

    // Check if running in development mode
    const isDev = import.meta.env.DEV;

    let invoke: any = null;

    onMount(async () => {
        // Dynamically import Tauri API to avoid SSR issues
        try {
            const tauri = await import("@tauri-apps/api/tauri");
            invoke = tauri.invoke;
            isTauriAvailable = true;
            await checkPrinterConnection();
        } catch {
            // Not running in Tauri or module not available
            isTauriAvailable = false;
        }

        // Load saved config
        const savedIp = localStorage.getItem("printer_ip");
        const savedPort = localStorage.getItem("printer_port");
        if (savedIp) printerIp = savedIp;
        if (savedPort) printerPort = savedPort;
    });

    async function checkPrinterConnection() {
        if (!isTauriAvailable || !invoke) {
            printerResult = {
                connected: false,
                printers: [],
                default_printer: null,
                error: "กรุณาเปิดแอปผ่าน Tauri เพื่อใช้งานฟีเจอร์นี้",
            };
            return;
        }

        isChecking = true;

        try {
            printerResult = (await invoke("check_printer_connection", {
                printerIp: printerIp || null,
                printerPort: printerPort || null,
            })) as PrinterCheckResult;
        } catch (err) {
            printerResult = {
                connected: false,
                printers: [],
                default_printer: null,
                error: `เกิดข้อผิดพลาด: ${err}`,
            };
        }

        isChecking = false;
    }

    async function testPrint() {
        if (!isTauriAvailable || !invoke) {
            testPrintMessage = "กรุณาเปิดแอปผ่าน Tauri เพื่อใช้งานฟีเจอร์นี้";
            showTestResult = true;
            return;
        }

        isPrinting = true;
        showTestResult = false;

        try {
            const result = (await invoke("test_print", {
                printerIp: printerIp || null,
                printerPort: printerPort || null,
            })) as TestPrintResult;
            testPrintMessage = result.message;
            showTestResult = true;

            // Refresh printer status
            if (result.success) {
                await checkPrinterConnection();
            }
        } catch (err) {
            testPrintMessage = `เกิดข้อผิดพลาด: ${err}`;
            showTestResult = true;
        }

        isPrinting = false;

        // Hide message after 5 seconds
        setTimeout(() => {
            showTestResult = false;
            testPrintMessage = "";
        }, 5000);
    }

    function getStatusColor(connected: boolean, error: string | null): string {
        if (error || !connected) return "var(--color-danger)";
        return "var(--color-success)";
    }

    function getStatusIcon(
        connected: boolean,
        error: string | null,
        checking: boolean,
    ): string {
        if (checking) return "🔄";
        if (error || !connected) return "❌";
        return "✅";
    }
    function saveNetworkConfig() {
        localStorage.setItem("printer_ip", printerIp);
        localStorage.setItem("printer_port", printerPort);
        isNetworkVerified = false; // Reset verification on change
    }

    async function verifyNetworkPrinter() {
        if (!printerIp || !printerPort) {
            alert("กรุณาระบุ IP และ Port");
            return;
        }
        await checkPrinterConnection();

        const networkPrinter = printerResult.printers.find((p) =>
            p.name.includes(printerIp),
        );
        if (networkPrinter) {
            isNetworkVerified = true;
            alert(`✅ เชื่อมต่อสำเร็จ: ${networkPrinter.name}`);
        } else {
            isNetworkVerified = false;
            alert(
                "❌ ไม่สามารถเชื่อมต่อได้ กรุณาตรวจสอบ IP/Port และการเชื่อมต่อ",
            );
        }
    }
</script>

<div class="printer-config">
    <div class="config-header">
        <h2>🖨️ ตั้งค่าเครื่องพิมพ์</h2>
        <p class="header-desc">
            ตรวจสอบและทดสอบการเชื่อมต่อเครื่องพิมพ์ใบเสร็จ
        </p>
    </div>

    {#if !isTauriAvailable}
        <div class="tauri-warning">
            <p>⚠️ ฟีเจอร์นี้ใช้ได้เฉพาะเมื่อเปิดแอปผ่าน Tauri (Desktop App)</p>
            <p class="hint">
                กรุณา build และรันแอปผ่าน Tauri
                เพื่อใช้งานการตรวจสอบเครื่องพิมพ์
            </p>
        </div>
    {:else}
        <div class="printer-status-card">
            <div class="status-header">
                <div class="status-icon">
                    <span class:spinner={isChecking}>
                        {getStatusIcon(
                            printerResult.connected,
                            printerResult.error,
                            isChecking,
                        )}
                    </span>
                </div>
                <div class="status-info">
                    <div class="status-title-row">
                        <h3>สถานะเครื่องพิมพ์</h3>
                    </div>
                    <p
                        class="status-text"
                        style="color: {getStatusColor(
                            printerResult.connected,
                            printerResult.error,
                        )}"
                    >
                        {#if isChecking}
                            กำลังตรวจสอบ...
                        {:else if printerResult.error}
                            {printerResult.error}
                        {:else if printerResult.connected}
                            พร้อมใช้งาน
                        {:else}
                            ไม่พบเครื่องพิมพ์
                        {/if}
                    </p>
                </div>
            </div>

            {#if printerResult.connected && printerResult.printers.length > 0}
                <div class="printer-list">
                    <h4>เครื่องพิมพ์ที่พบ ({printerResult.printers.length})</h4>
                    {#each printerResult.printers as printer}
                        <div
                            class="printer-item"
                            class:default={printer.is_default}
                        >
                            <div class="printer-name">
                                {#if printer.is_default}
                                    <span class="default-badge">⭐</span>
                                {/if}
                                {printer.name}
                            </div>
                            <div class="printer-status">{printer.status}</div>
                        </div>
                    {/each}
                </div>
            {/if}

            {#if !printerResult.connected && !isChecking && !printerResult.error}
                <div class="no-printer-message">
                    <p>⚠️ ไม่พบเครื่องพิมพ์ที่เชื่อมต่อ</p>
                    <ul>
                        <li>ตรวจสอบว่าเครื่องพิมพ์เปิดอยู่</li>
                        <li>ตรวจสอบสาย USB หรือการเชื่อมต่อ WiFi</li>
                        <li>ติดตั้งไดรเวอร์เครื่องพิมพ์</li>
                    </ul>
                </div>
            {/if}
        </div>

        <div class="action-buttons">
            <button
                class="btn btn-primary"
                onclick={checkPrinterConnection}
                disabled={isChecking}
            >
                {#if isChecking}
                    <span class="btn-spinner"></span>
                    กำลังตรวจสอบ...
                {:else}
                    🔄 ตรวจสอบการเชื่อมต่อ
                {/if}
            </button>

            <button
                class="btn btn-success"
                onclick={testPrint}
                disabled={isPrinting || !printerResult.connected}
            >
                {#if isPrinting}
                    <span class="btn-spinner"></span>
                    กำลังพิมพ์...
                {:else}
                    🧾 พิมพ์ทดสอบ
                {/if}
            </button>
        </div>

        {#if showTestResult}
            <div
                class="test-result"
                class:success={testPrintMessage.includes("สำเร็จ")}
                class:error={!testPrintMessage.includes("สำเร็จ")}
            >
                {testPrintMessage}
            </div>
        {/if}
    {/if}

    <div class="printer-tips">
        <div class="network-header">
            <h4>🌐 ตั้งค่า Network Printer</h4>
            {#if isNetworkVerified}
                <span class="verified-badge">✅ Connected</span>
            {/if}
        </div>
        <div class="network-config">
            <div class="form-group">
                <label for="printer-ip">IP Address</label>
                <input
                    type="text"
                    id="printer-ip"
                    placeholder="192.168.1.100"
                    bind:value={printerIp}
                    oninput={saveNetworkConfig}
                />
            </div>
            <div class="form-group">
                <label for="printer-port">Port</label>
                <div class="input-group">
                    <input
                        type="text"
                        id="printer-port"
                        placeholder="9100"
                        bind:value={printerPort}
                        oninput={saveNetworkConfig}
                    />
                    <button
                        class="btn-verify"
                        onclick={verifyNetworkPrinter}
                        disabled={isChecking}
                    >
                        {isChecking ? "..." : "ตรวจสอบ"}
                    </button>
                </div>
            </div>
        </div>
    </div>

    <div class="printer-tips">
        <h4>💡 คำแนะนำ</h4>
        <ul>
            <li>
                ใช้เครื่องปริ้นเตอร์ (Printer) ขนาด 58mm หรือ 80mm สำหรับใบเสร็จ
            </li>
            <li>ตรวจสอบให้แน่ใจว่ามีกระดาษเพียงพอ</li>
            <li>หากพิมพ์ไม่ได้ ลองรีสตาร์ทเครื่องพิมพ์</li>
            <li>ต้องรันแอปผ่าน Tauri (Desktop) เพื่อเข้าถึงเครื่องพิมพ์จริง</li>
        </ul>
    </div>
</div>

<style>
    .printer-config {
        max-width: 600px;
        margin: 0 auto;
        padding: var(--space-6);
    }

    .config-header {
        text-align: center;
        margin-bottom: var(--space-8);
    }

    .config-header h2 {
        font-size: 1.75rem;
        color: var(--color-text-primary);
        margin-bottom: var(--space-2);
    }

    .header-desc {
        color: var(--color-text-muted);
        font-size: 0.9375rem;
    }

    .tauri-warning {
        background: var(--color-warning-bg, rgba(234, 179, 8, 0.1));
        border: 1px solid var(--color-warning, #eab308);
        border-radius: var(--radius-lg);
        padding: var(--space-6);
        text-align: center;
        margin-bottom: var(--space-6);
    }

    .tauri-warning p {
        color: var(--color-warning, #eab308);
        margin: 0;
        font-weight: 600;
    }

    .tauri-warning .hint {
        color: var(--color-text-muted);
        font-weight: 400;
        font-size: 0.875rem;
        margin-top: var(--space-2);
    }

    .printer-status-card {
        background: var(--color-bg-secondary);
        border: 1px solid var(--color-bg-hover);
        border-radius: var(--radius-xl);
        padding: var(--space-6);
        margin-bottom: var(--space-6);
    }

    .status-header {
        display: flex;
        align-items: center;
        gap: var(--space-4);
        margin-bottom: var(--space-4);
    }

    .status-icon {
        width: 64px;
        height: 64px;
        display: flex;
        align-items: center;
        justify-content: center;
        background: var(--color-bg-tertiary);
        border-radius: var(--radius-lg);
        font-size: 2rem;
    }

    .status-icon .spinner {
        animation: spin 1s linear infinite;
    }

    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }

    .status-info h3 {
        font-size: 1.125rem;
        color: var(--color-text-primary);
        margin-bottom: var(--space-1);
    }

    .status-text {
        font-weight: 600;
        font-size: 0.9375rem;
    }

    .printer-list {
        margin-top: var(--space-4);
        background: var(--color-bg-tertiary);
        border-radius: var(--radius-md);
        padding: var(--space-4);
    }

    .printer-list h4 {
        font-size: 0.875rem;
        color: var(--color-text-muted);
        margin-bottom: var(--space-3);
    }

    .printer-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: var(--space-3);
        background: var(--color-bg-secondary);
        border-radius: var(--radius-md);
        margin-bottom: var(--space-2);
    }

    .printer-item:last-child {
        margin-bottom: 0;
    }

    .printer-item.default {
        border: 1px solid var(--color-primary);
    }

    .printer-name {
        display: flex;
        align-items: center;
        gap: var(--space-2);
        color: var(--color-text-primary);
        font-weight: 500;
    }

    .default-badge {
        font-size: 0.875rem;
    }

    .printer-status {
        font-size: 0.75rem;
        color: var(--color-success);
        background: var(--color-success-bg);
        padding: var(--space-1) var(--space-2);
        border-radius: var(--radius-full);
    }

    .no-printer-message {
        background: var(--color-danger-bg);
        border: 1px solid var(--color-danger);
        border-radius: var(--radius-md);
        padding: var(--space-4);
        margin-top: var(--space-4);
    }

    .no-printer-message p {
        color: var(--color-danger);
        font-weight: 600;
        margin-bottom: var(--space-2);
    }

    .no-printer-message ul {
        color: var(--color-text-secondary);
        padding-left: var(--space-5);
        margin: 0;
    }

    .no-printer-message li {
        margin-bottom: var(--space-1);
        font-size: 0.875rem;
    }

    .action-buttons {
        display: flex;
        gap: var(--space-4);
        margin-bottom: var(--space-6);
    }

    .action-buttons .btn {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: var(--space-2);
        padding: var(--space-4);
        font-size: 1rem;
    }

    .btn-spinner {
        width: 16px;
        height: 16px;
        border: 2px solid transparent;
        border-top-color: currentColor;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
    }

    .test-result {
        padding: var(--space-4);
        border-radius: var(--radius-md);
        text-align: center;
        font-weight: 500;
        margin-bottom: var(--space-6);
        animation: fadeIn 0.3s ease;
    }

    .test-result.success {
        background: var(--color-success-bg);
        color: var(--color-success);
        border: 1px solid var(--color-success);
    }

    .test-result.error {
        background: var(--color-danger-bg);
        color: var(--color-danger);
        border: 1px solid var(--color-danger);
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
            transform: translateY(-10px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    .printer-tips {
        background: var(--color-bg-secondary);
        border: 1px solid var(--color-bg-hover);
        border-radius: var(--radius-lg);
        padding: var(--space-5);
    }

    .printer-tips h4 {
        color: var(--color-primary);
        margin-bottom: var(--space-3);
        font-size: 1rem;
    }

    .printer-tips ul {
        color: var(--color-text-secondary);
        padding-left: var(--space-5);
        margin: 0;
    }

    .printer-tips li {
        margin-bottom: var(--space-2);
        font-size: 0.875rem;
    }

    .network-config {
        display: flex;
        gap: var(--space-4);
        margin-top: var(--space-2);
    }

    .form-group {
        flex: 1;
        display: flex;
        flex-direction: column;
        gap: var(--space-2);
    }

    .form-group label {
        font-size: 0.875rem;
        font-weight: 500;
        color: var(--color-text-secondary);
    }

    .form-group input {
        padding: var(--space-2) var(--space-3);
        border: 1px solid var(--color-bg-hover);
        border-radius: var(--radius-md);
        background: var(--color-bg-primary);
        color: var(--color-text-primary);
        font-size: 0.9375rem;
    }

    .form-group input:focus {
        outline: none;
        border-color: var(--color-primary);
        box-shadow: 0 0 0 2px var(--color-primary-bg);
    }

    .input-group {
        display: flex;
        gap: var(--space-2);
    }

    .input-group input {
        flex: 1;
    }

    .btn-verify {
        background: var(--color-primary);
        color: white;
        border: none;
        border-radius: var(--radius-md);
        padding: 0 var(--space-3);
        font-size: 0.875rem;
        cursor: pointer;
        transition: background 0.2s;
        white-space: nowrap;
    }

    .btn-verify:hover {
        background: var(--color-primary-hover);
    }

    .btn-verify:disabled {
        opacity: 0.7;
        cursor: not-allowed;
    }

    .network-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: var(--space-3);
    }

    .network-header h4 {
        margin: 0;
    }

    .verified-badge {
        font-size: 0.75rem;
        color: var(--color-success);
        background: var(--color-success-bg);
        border: 1px solid var(--color-success);
        padding: 2px 8px;
        border-radius: 12px;
        font-weight: 600;
    }
</style>
