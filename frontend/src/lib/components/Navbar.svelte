<script lang="ts">
    import { currentUser } from "../stores";
    import { page } from "$app/stores";

    let { title = "Pot Cafe" }: { title?: string } = $props();

    let isSettingsPage = $derived($page.url.pathname.startsWith("/settings"));

    function handleLogout() {
        currentUser.logout();
        window.location.href = "/";
    }
</script>

<nav class="navbar">
    <div class="navbar-brand">
        <span class="logo">☕</span>
        <span class="brand-name">{title}</span>
    </div>

    {#if $currentUser}
        <div class="navbar-user">
            {#if !isSettingsPage}
                <a href="/settings" class="settings-btn" title="ตั้งค่าระบบ">
                    ⚙️
                </a>
            {/if}
            <div class="user-info">
                <span class="user-avatar">👤</span>
                <span class="user-name">{$currentUser.name}</span>
                <span class="user-role"
                    >{$currentUser.role === "admin"
                        ? "ผู้ดูแล"
                        : "พนักงาน"}</span
                >
            </div>
            <button class="btn btn-ghost btn-sm" onclick={handleLogout}>
                ออกจากระบบ
            </button>
        </div>
    {/if}
</nav>

<style>
    .navbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: var(--space-4) var(--space-6);
        background: var(--color-bg-secondary);
        border-bottom: 1px solid var(--color-bg-hover);
    }

    .navbar-brand {
        display: flex;
        align-items: center;
        gap: var(--space-3);
    }

    .logo {
        font-size: 1.75rem;
    }

    .brand-name {
        font-family: var(--font-display);
        font-size: 1.5rem;
        font-weight: 700;
        color: var(--color-primary);
    }

    .navbar-user {
        display: flex;
        align-items: center;
        gap: var(--space-4);
    }

    .user-info {
        display: flex;
        align-items: center;
        gap: var(--space-2);
        padding: var(--space-2) var(--space-3);
        background: var(--color-bg-tertiary);
        border-radius: var(--radius-full);
    }

    .user-avatar {
        font-size: 1.25rem;
    }

    .user-name {
        font-weight: 500;
        color: var(--color-text-primary);
    }

    .user-role {
        font-size: 0.75rem;
        color: var(--color-text-muted);
        padding: var(--space-1) var(--space-2);
        background: var(--color-bg-hover);
        border-radius: var(--radius-full);
    }

    .settings-btn {
        display: flex;
        align-items: center;
        justify-content: center;
        width: 40px;
        height: 40px;
        background: var(--color-bg-tertiary);
        border-radius: var(--radius-md);
        font-size: 1.25rem;
        text-decoration: none;
        transition: all var(--transition-fast);
    }

    .settings-btn:hover {
        background: var(--color-bg-hover);
        transform: rotate(45deg);
    }
</style>
