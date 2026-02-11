<script lang="ts">
    import Navbar from "$lib/components/Navbar.svelte";
    import PrinterConfig from "$lib/components/PrinterConfig.svelte";
    import { currentUser } from "$lib/stores";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";

    let activeTab = $state("printer");

    const tabs = [
        { id: "printer", name: "เครื่องพิมพ์", icon: "🖨️" },
        { id: "general", name: "ทั่วไป", icon: "⚙️" },
    ];

    onMount(() => {
        const unsubscribe = currentUser.subscribe((user) => {
            if (!user) {
                goto("/");
            }
        });
        return unsubscribe;
    });
</script>

<div class="settings-layout">
    <Navbar title="ตั้งค่าระบบ" />

    <div class="settings-main">
        <aside class="settings-sidebar">
            <div class="sidebar-header">
                <h3>⚙️ การตั้งค่า</h3>
            </div>
            <nav class="settings-nav">
                {#each tabs as tab}
                    <button
                        class="nav-item"
                        class:active={activeTab === tab.id}
                        onclick={() => (activeTab = tab.id)}
                    >
                        <span class="nav-icon">{tab.icon}</span>
                        <span class="nav-text">{tab.name}</span>
                    </button>
                {/each}
            </nav>

            <div class="sidebar-footer">
                <a href="/menu" class="btn btn-ghost back-btn">
                    ← กลับหน้าขาย
                </a>
            </div>
        </aside>

        <main class="settings-content">
            {#if activeTab === "printer"}
                <PrinterConfig />
            {:else if activeTab === "general"}
                <div class="general-settings">
                    <h2>⚙️ ตั้งค่าทั่วไป</h2>
                    <p class="coming-soon">เร็วๆ นี้...</p>
                </div>
            {/if}
        </main>
    </div>
</div>

<style>
    .settings-layout {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
        background: var(--color-bg-primary);
    }

    .settings-main {
        flex: 1;
        display: grid;
        grid-template-columns: 280px 1fr;
        overflow: hidden;
    }

    .settings-sidebar {
        display: flex;
        flex-direction: column;
        background: var(--color-bg-secondary);
        border-right: 1px solid var(--color-bg-hover);
    }

    .sidebar-header {
        padding: var(--space-6);
        border-bottom: 1px solid var(--color-bg-hover);
    }

    .sidebar-header h3 {
        font-size: 1.25rem;
        color: var(--color-text-primary);
        margin: 0;
    }

    .settings-nav {
        flex: 1;
        padding: var(--space-4);
        display: flex;
        flex-direction: column;
        gap: var(--space-2);
    }

    .nav-item {
        display: flex;
        align-items: center;
        gap: var(--space-3);
        padding: var(--space-3) var(--space-4);
        background: transparent;
        border: none;
        border-radius: var(--radius-md);
        color: var(--color-text-secondary);
        font-family: inherit;
        font-size: 0.9375rem;
        cursor: pointer;
        transition: all var(--transition-fast);
        text-align: left;
    }

    .nav-item:hover {
        background: var(--color-bg-hover);
        color: var(--color-text-primary);
    }

    .nav-item.active {
        background: linear-gradient(
            135deg,
            var(--color-primary) 0%,
            var(--color-primary-dark) 100%
        );
        color: var(--color-bg-primary);
    }

    .nav-icon {
        font-size: 1.25rem;
    }

    .sidebar-footer {
        padding: var(--space-4);
        border-top: 1px solid var(--color-bg-hover);
    }

    .back-btn {
        width: 100%;
        justify-content: center;
        text-decoration: none;
    }

    .settings-content {
        overflow-y: auto;
        padding: var(--space-6);
    }

    .general-settings {
        text-align: center;
        padding: var(--space-10);
    }

    .general-settings h2 {
        color: var(--color-text-primary);
        margin-bottom: var(--space-4);
    }

    .coming-soon {
        color: var(--color-text-muted);
        font-size: 1.125rem;
    }
</style>
