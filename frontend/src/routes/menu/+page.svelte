<script lang="ts">
    import Navbar from "$lib/components/Navbar.svelte";
    import ProductCard from "$lib/components/ProductCard.svelte";
    import CartItem from "$lib/components/CartItem.svelte";
    import {
        fetchCategories,
        fetchMenuItems,
        type Category,
    } from "$lib/data/menu";
    import {
        cart,
        cartTotal,
        cartItemCount,
        selectedCategory,
        currentUser,
        selectedPaymentMethod,
        type PaymentMethod,
        type MenuItem,
    } from "$lib/stores";
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";

    let showPaymentModal = $state(false);
    let showConfirmModal = $state(false);
    let orderNumber = $state(0);
    let invoke: any = null;

    // DB-driven data
    let dbCategories = $state<Category[]>([]);
    let currentProducts = $state<MenuItem[]>([]);

    async function loadProducts(category: string) {
        currentProducts = await fetchMenuItems(category);
    }

    onMount(() => {
        // Check if user is logged in
        const unsubscribe = currentUser.subscribe((user) => {
            if (!user) {
                goto("/");
            }
        });

        // Load Tauri invoke
        import("@tauri-apps/api/tauri")
            .then((tauri) => {
                invoke = tauri.invoke;
            })
            .catch(() => {
                /* Browser mode */
            });

        // Load categories and initial products from DB
        fetchCategories().then((cats) => {
            dbCategories = cats;
        });
        loadProducts($selectedCategory);

        return unsubscribe;
    });

    function selectCategory(categoryId: string) {
        selectedCategory.set(categoryId);
        loadProducts(categoryId);
    }

    function selectPayment(method: PaymentMethod) {
        selectedPaymentMethod.set(method);
    }

    function openPayment() {
        if ($cartItemCount > 0) {
            showPaymentModal = true;
        }
    }

    async function confirmOrder() {
        showPaymentModal = false;

        if (invoke && $currentUser) {
            try {
                // Save order to SQLite via Tauri
                const items = $cart.map((entry) => ({
                    item_id: entry.item.id,
                    item_name: entry.item.name,
                    qty: entry.qty,
                    price: entry.item.price,
                }));

                const result = await invoke("create_order", {
                    items,
                    total: $cartTotal,
                    paymentMethod: $selectedPaymentMethod,
                    cashierId: $currentUser.id,
                    cashierName: $currentUser.name,
                });

                if (result.success) {
                    orderNumber = result.order_number;
                } else {
                    orderNumber = Math.floor(Math.random() * 9000) + 1000;
                    console.error("Order save failed:", result.error);
                }
            } catch (err) {
                orderNumber = Math.floor(Math.random() * 9000) + 1000;
                console.error("Order save error:", err);
            }
        } else {
            // Browser fallback
            orderNumber = Math.floor(Math.random() * 9000) + 1000;
        }

        showConfirmModal = true;
    }

    function closeConfirmModal() {
        showConfirmModal = false;
        cart.clear();
    }

    const paymentMethods = [
        { id: "cash", name: "เงินสด", icon: "💵" },
        { id: "promptpay", name: "พร้อมเพย์", icon: "📱" },
        { id: "card", name: "บัตรเครดิต", icon: "💳" },
    ] as const;
</script>

<div class="pos-layout">
    <Navbar title="Pot Cafe POS" />

    <div class="pos-main">
        <!-- Products Section -->
        <div class="products-section">
            <!-- Category Tabs -->
            <div class="category-tabs">
                {#each dbCategories as category}
                    <button
                        class="category-tab"
                        class:active={$selectedCategory === category.id}
                        onclick={() => selectCategory(category.id)}
                    >
                        <span class="tab-icon">{category.icon}</span>
                        <span class="tab-name">{category.name}</span>
                    </button>
                {/each}
            </div>

            <!-- Product Grid -->
            <div class="products-grid">
                {#each currentProducts as product (product.id)}
                    <ProductCard item={product} />
                {/each}
            </div>
        </div>

        <!-- Cart Sidebar -->
        <aside class="cart-sidebar">
            <div class="cart-header">
                <h2>🛒 ตะกร้าสินค้า</h2>
                {#if $cartItemCount > 0}
                    <span class="cart-count">{$cartItemCount}</span>
                {/if}
            </div>

            <div class="cart-items">
                {#if $cart.length === 0}
                    <div class="cart-empty">
                        <span class="empty-icon">🛒</span>
                        <p>ตะกร้าว่าง</p>
                        <p class="empty-hint">เลือกสินค้าเพื่อเพิ่มในตะกร้า</p>
                    </div>
                {:else}
                    {#each $cart as entry (entry.item.id)}
                        <CartItem {entry} />
                    {/each}
                {/if}
            </div>

            <div class="cart-footer">
                <div class="cart-summary">
                    <div class="summary-row">
                        <span>รายการทั้งหมด</span>
                        <span>{$cartItemCount} ชิ้น</span>
                    </div>
                    <div class="summary-row total">
                        <span>ยอดรวม</span>
                        <span class="total-amount">{$cartTotal}฿</span>
                    </div>
                </div>

                <button
                    class="btn btn-success btn-lg checkout-btn"
                    disabled={$cartItemCount === 0}
                    onclick={openPayment}
                >
                    ✓ ชำระเงิน
                </button>
            </div>
        </aside>
    </div>
</div>

<!-- Payment Modal -->
{#if showPaymentModal}
    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
    <div
        class="modal-overlay"
        onclick={() => (showPaymentModal = false)}
        onkeydown={(e) => e.key === "Escape" && (showPaymentModal = false)}
        role="dialog"
        aria-modal="true"
        tabindex="-1"
    >
        <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
        <div
            class="modal payment-modal"
            onclick={(e) => e.stopPropagation()}
            onkeydown={(e) => e.stopPropagation()}
            role="document"
        >
            <div class="modal-header">
                <h2>💰 เลือกวิธีชำระเงิน</h2>
            </div>
            <div class="modal-body">
                <div class="payment-summary">
                    <p>ยอดที่ต้องชำระ</p>
                    <p class="payment-amount">{$cartTotal}฿</p>
                </div>

                <div class="payment-methods">
                    {#each paymentMethods as method}
                        <button
                            class="payment-method"
                            class:selected={$selectedPaymentMethod ===
                                method.id}
                            onclick={() => selectPayment(method.id)}
                        >
                            <span class="method-icon">{method.icon}</span>
                            <span class="method-name">{method.name}</span>
                        </button>
                    {/each}
                </div>
            </div>
            <div class="modal-footer">
                <button
                    class="btn btn-ghost"
                    onclick={() => (showPaymentModal = false)}
                >
                    ยกเลิก
                </button>
                <button class="btn btn-success" onclick={confirmOrder}>
                    ✓ ยืนยันการชำระ
                </button>
            </div>
        </div>
    </div>
{/if}

<!-- Order Confirmation Modal -->
{#if showConfirmModal}
    <div class="modal-overlay" role="dialog" aria-modal="true">
        <div class="modal confirm-modal animate-scale-in" role="document">
            <div class="confirm-content">
                <div class="success-icon">✅</div>
                <h2>สั่งซื้อสำเร็จ!</h2>
                <p class="order-number">
                    หมายเลขออเดอร์: <strong>#{orderNumber}</strong>
                </p>
                <div class="order-summary">
                    <p>ยอดชำระ: <strong>{$cartTotal}฿</strong></p>
                    <p>
                        วิธีชำระ: <strong>
                            {paymentMethods.find(
                                (m) => m.id === $selectedPaymentMethod,
                            )?.name}
                        </strong>
                    </p>
                </div>
                <button
                    class="btn btn-primary btn-lg"
                    onclick={closeConfirmModal}
                >
                    รับออเดอร์ใหม่
                </button>
            </div>
        </div>
    </div>
{/if}

<style>
    .pos-layout {
        min-height: 100vh;
        display: flex;
        flex-direction: column;
        background: var(--color-bg-primary);
    }

    .pos-main {
        flex: 1;
        display: grid;
        grid-template-columns: 1fr 380px;
        gap: 0;
        overflow: hidden;
    }

    /* Products Section */
    .products-section {
        display: flex;
        flex-direction: column;
        overflow: hidden;
    }

    .category-tabs {
        display: flex;
        gap: var(--space-3);
        padding: var(--space-4) var(--space-6);
        background: var(--color-bg-secondary);
        border-bottom: 1px solid var(--color-bg-hover);
        overflow-x: auto;
    }

    .category-tab {
        display: flex;
        align-items: center;
        gap: var(--space-2);
        padding: var(--space-3) var(--space-5);
        background: var(--color-bg-tertiary);
        border: 1px solid var(--color-bg-hover);
        border-radius: var(--radius-full);
        color: var(--color-text-secondary);
        font-family: inherit;
        font-size: 0.9375rem;
        font-weight: 500;
        cursor: pointer;
        transition: all var(--transition-fast);
        white-space: nowrap;
    }

    .category-tab:hover {
        background: var(--color-bg-hover);
        color: var(--color-text-primary);
    }

    .category-tab.active {
        background: linear-gradient(
            135deg,
            var(--color-primary) 0%,
            var(--color-primary-dark) 100%
        );
        border-color: var(--color-primary);
        color: var(--color-bg-primary);
        box-shadow: var(--shadow-glow);
    }

    .tab-icon {
        font-size: 1.25rem;
    }

    .products-grid {
        flex: 1;
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
        gap: var(--space-4);
        padding: var(--space-6);
        overflow-y: auto;
        align-content: start;
    }

    /* Cart Sidebar */
    .cart-sidebar {
        display: flex;
        flex-direction: column;
        background: var(--color-bg-secondary);
        border-left: 1px solid var(--color-bg-hover);
        overflow: hidden;
    }

    .cart-header {
        display: flex;
        align-items: center;
        gap: var(--space-3);
        padding: var(--space-4) var(--space-5);
        border-bottom: 1px solid var(--color-bg-hover);
    }

    .cart-header h2 {
        font-size: 1.125rem;
        margin: 0;
    }

    .cart-count {
        background: var(--color-primary);
        color: var(--color-bg-primary);
        padding: var(--space-1) var(--space-2);
        border-radius: var(--radius-full);
        font-size: 0.75rem;
        font-weight: 600;
        min-width: 24px;
        text-align: center;
    }

    .cart-items {
        flex: 1;
        overflow-y: auto;
        padding: var(--space-4);
        display: flex;
        flex-direction: column;
        gap: var(--space-3);
    }

    .cart-empty {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        text-align: center;
        padding: var(--space-8);
    }

    .empty-icon {
        font-size: 3rem;
        opacity: 0.3;
        margin-bottom: var(--space-4);
    }

    .cart-empty p {
        color: var(--color-text-muted);
        margin: 0;
    }

    .empty-hint {
        font-size: 0.875rem;
        margin-top: var(--space-2) !important;
    }

    .cart-footer {
        padding: var(--space-4) var(--space-5);
        border-top: 1px solid var(--color-bg-hover);
        background: var(--color-bg-tertiary);
    }

    .cart-summary {
        margin-bottom: var(--space-4);
    }

    .summary-row {
        display: flex;
        justify-content: space-between;
        padding: var(--space-2) 0;
        color: var(--color-text-secondary);
        font-size: 0.875rem;
    }

    .summary-row.total {
        border-top: 1px solid var(--color-bg-hover);
        padding-top: var(--space-3);
        margin-top: var(--space-2);
        font-weight: 600;
        color: var(--color-text-primary);
    }

    .total-amount {
        font-size: 1.5rem;
        color: var(--color-primary);
    }

    .checkout-btn {
        width: 100%;
    }

    /* Payment Modal */
    .payment-modal {
        width: 100%;
        max-width: 450px;
    }

    .payment-summary {
        text-align: center;
        padding: var(--space-6);
        background: var(--color-bg-tertiary);
        border-radius: var(--radius-lg);
        margin-bottom: var(--space-6);
    }

    .payment-summary p:first-child {
        color: var(--color-text-muted);
        margin-bottom: var(--space-2);
    }

    .payment-amount {
        font-size: 2.5rem;
        font-weight: 700;
        color: var(--color-primary);
        margin: 0;
    }

    .payment-methods {
        display: flex;
        flex-direction: column;
        gap: var(--space-3);
    }

    .payment-method {
        display: flex;
        align-items: center;
        gap: var(--space-4);
        padding: var(--space-4) var(--space-5);
        background: var(--color-bg-tertiary);
        border: 2px solid var(--color-bg-hover);
        border-radius: var(--radius-lg);
        cursor: pointer;
        transition: all var(--transition-fast);
        font-family: inherit;
        text-align: left;
    }

    .payment-method:hover {
        border-color: var(--color-primary-dark);
        background: var(--color-bg-hover);
    }

    .payment-method.selected {
        border-color: var(--color-primary);
        background: rgba(196, 167, 125, 0.1);
    }

    .method-icon {
        font-size: 1.75rem;
    }

    .method-name {
        font-size: 1rem;
        font-weight: 500;
        color: var(--color-text-primary);
    }

    /* Confirm Modal */
    .confirm-modal {
        width: 100%;
        max-width: 400px;
        text-align: center;
    }

    .confirm-content {
        padding: var(--space-8);
    }

    .success-icon {
        font-size: 4rem;
        margin-bottom: var(--space-4);
    }

    .confirm-content h2 {
        font-size: 1.75rem;
        margin-bottom: var(--space-4);
        color: var(--color-success);
    }

    .order-number {
        font-size: 1.125rem;
        color: var(--color-text-secondary);
        margin-bottom: var(--space-4);
    }

    .order-number strong {
        color: var(--color-primary);
    }

    .order-summary {
        background: var(--color-bg-tertiary);
        padding: var(--space-4);
        border-radius: var(--radius-md);
        margin-bottom: var(--space-6);
    }

    .order-summary p {
        margin: var(--space-2) 0;
        color: var(--color-text-secondary);
    }

    .order-summary strong {
        color: var(--color-text-primary);
    }
</style>
