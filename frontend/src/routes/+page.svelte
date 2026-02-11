<script lang="ts">
    import { currentUser } from "$lib/stores";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    let userId = $state("");
    let password = $state("");
    let error = $state("");
    let isLoading = $state(false);
    let invoke: any = null;

    onMount(async () => {
        try {
            const tauri = await import("@tauri-apps/api/tauri");
            invoke = tauri.invoke;
        } catch {
            // Running in browser — will use fallback
        }
    });

    async function handleLogin(e: Event) {
        e.preventDefault();
        error = "";

        if (!userId.trim()) {
            error = "กรุณากรอกรหัสพนักงาน";
            return;
        }
        if (!password.trim()) {
            error = "กรุณากรอกรหัสผ่าน";
            return;
        }

        isLoading = true;

        try {
            if (invoke) {
                // Real authentication via Tauri/SQLite
                const result = await invoke("login", {
                    userId: userId.trim(),
                    password: password.trim(),
                });

                if (result.success && result.user) {
                    currentUser.login({
                        id: result.user.id,
                        name: result.user.name,
                        role: result.user.role as "admin" | "cashier",
                    });
                    goto("/menu");
                } else {
                    error = result.error || "เข้าสู่ระบบไม่สำเร็จ";
                }
            } else {
                // Fallback for browser testing
                await new Promise((resolve) => setTimeout(resolve, 500));
                currentUser.login({
                    id: userId,
                    name:
                        userId === "admin"
                            ? "ผู้ดูแลระบบ"
                            : `พนักงาน ${userId}`,
                    role: userId === "admin" ? "admin" : "cashier",
                });
                goto("/menu");
            }
        } catch (err: any) {
            error = err?.message || "เกิดข้อผิดพลาดในการเข้าสู่ระบบ";
        } finally {
            isLoading = false;
        }
    }
</script>

<div class="login-container">
    <div class="login-background">
        <div class="bg-gradient"></div>
        <div class="bg-pattern"></div>
    </div>

    <div class="login-card animate-scale-in">
        <div class="login-header">
            <div class="logo-wrapper">
                <span class="logo">☕</span>
            </div>
            <h1>Pot Cafe</h1>
            <p>ระบบจัดการร้านกาแฟ</p>
        </div>

        <form class="login-form" onsubmit={handleLogin}>
            {#if error}
                <div class="error-message animate-fade-in">
                    <span>⚠️</span>
                    {error}
                </div>
            {/if}

            <div class="input-group">
                <label for="userId" class="input-label">รหัสพนักงาน</label>
                <div class="input-wrapper">
                    <span class="input-icon">👤</span>
                    <input
                        type="text"
                        id="userId"
                        class="input"
                        bind:value={userId}
                        placeholder="กรอกรหัสพนักงาน"
                        autocomplete="username"
                    />
                </div>
            </div>

            <div class="input-group">
                <label for="password" class="input-label">รหัสผ่าน</label>
                <div class="input-wrapper">
                    <span class="input-icon">🔒</span>
                    <input
                        type="password"
                        id="password"
                        class="input"
                        bind:value={password}
                        placeholder="กรอกรหัสผ่าน"
                        autocomplete="current-password"
                    />
                </div>
            </div>

            <button
                type="submit"
                class="btn btn-primary btn-lg login-btn"
                disabled={isLoading}
            >
                {#if isLoading}
                    <span class="spinner"></span>
                    กำลังเข้าสู่ระบบ...
                {:else}
                    เข้าสู่ระบบ
                {/if}
            </button>
        </form>

        <div class="login-footer">
            <p>Demo: ใส่ username/password อะไรก็ได้</p>
            <p>ใช้ "admin" เพื่อเข้าเป็นผู้ดูแล</p>
        </div>
    </div>
</div>

<style>
    .login-container {
        min-height: 100vh;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: var(--space-4);
        position: relative;
        overflow: hidden;
    }

    .login-background {
        position: absolute;
        inset: 0;
        z-index: 0;
    }

    .bg-gradient {
        position: absolute;
        inset: 0;
        background: radial-gradient(
                ellipse at top right,
                rgba(196, 167, 125, 0.15) 0%,
                transparent 50%
            ),
            radial-gradient(
                ellipse at bottom left,
                rgba(139, 105, 20, 0.1) 0%,
                transparent 50%
            );
    }

    .bg-pattern {
        position: absolute;
        inset: 0;
        opacity: 0.03;
        background-image: url("data:image/svg+xml,%3Csvg width='60' height='60' viewBox='0 0 60 60' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='none' fill-rule='evenodd'%3E%3Cg fill='%23C4A77D' fill-opacity='1'%3E%3Cpath d='M36 34v-4h-2v4h-4v2h4v4h2v-4h4v-2h-4zm0-30V0h-2v4h-4v2h4v4h2V6h4V4h-4zM6 34v-4H4v4H0v2h4v4h2v-4h4v-2H6zM6 4V0H4v4H0v2h4v4h2V6h4V4H6z'/%3E%3C/g%3E%3C/g%3E%3C/svg%3E");
    }

    .login-card {
        position: relative;
        z-index: 1;
        background: var(--color-bg-secondary);
        border: 1px solid var(--color-bg-hover);
        border-radius: var(--radius-xl);
        padding: var(--space-10);
        width: 100%;
        max-width: 420px;
        box-shadow: var(--shadow-xl);
    }

    .login-header {
        text-align: center;
        margin-bottom: var(--space-8);
    }

    .logo-wrapper {
        width: 80px;
        height: 80px;
        margin: 0 auto var(--space-4);
        background: linear-gradient(
            135deg,
            var(--color-primary) 0%,
            var(--color-primary-dark) 100%
        );
        border-radius: var(--radius-xl);
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: var(--shadow-glow);
    }

    .logo {
        font-size: 2.5rem;
    }

    .login-header h1 {
        font-family: var(--font-display);
        font-size: 2rem;
        color: var(--color-primary);
        margin-bottom: var(--space-2);
    }

    .login-header p {
        color: var(--color-text-muted);
        font-size: 0.9375rem;
    }

    .login-form {
        display: flex;
        flex-direction: column;
        gap: var(--space-5);
    }

    .input-wrapper {
        position: relative;
    }

    .input-icon {
        position: absolute;
        left: var(--space-4);
        top: 50%;
        transform: translateY(-50%);
        font-size: 1rem;
        opacity: 0.7;
    }

    .input-wrapper .input {
        padding-left: 48px;
    }

    .error-message {
        display: flex;
        align-items: center;
        gap: var(--space-2);
        padding: var(--space-3) var(--space-4);
        background: var(--color-danger-bg);
        border: 1px solid var(--color-danger);
        border-radius: var(--radius-md);
        color: var(--color-danger);
        font-size: 0.875rem;
    }

    .login-btn {
        width: 100%;
        margin-top: var(--space-2);
    }

    .spinner {
        width: 18px;
        height: 18px;
        border: 2px solid transparent;
        border-top-color: currentColor;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    .login-footer {
        margin-top: var(--space-6);
        padding-top: var(--space-6);
        border-top: 1px solid var(--color-bg-hover);
        text-align: center;
    }

    .login-footer p {
        font-size: 0.8125rem;
        color: var(--color-text-muted);
        margin-bottom: var(--space-1);
    }
</style>
