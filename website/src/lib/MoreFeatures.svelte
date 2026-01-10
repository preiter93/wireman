<script lang="ts">
    import ImgOpenEditor from '$lib/assets/website_249.png';
    import ImgResponse from '$lib/assets/website_277.png';
    import ImgConfig from '$lib/assets/website_460.png';
    import ImgReflection from '$lib/assets/website_526.png';

    type ShowElement = {
        title: string;
        img: string;
        // TODO: Use snippets
        body: string;
    };

    let elements: ShowElement[] = [
        {
            title: 'Open in system editor',
            img: ImgOpenEditor,
            body: `
          <p style="text-align: center; line-height:1.6em">
              Press <code>&ltC-e&gt</code> in any input field in normal mode to open the content in your system editor.
          </p>
`
        },
        {
            title: 'Save history',
            img: ImgResponse,
            body: `
            <p style="text-align: center; line-height:1.6em">
Sent requests are automatically saved. To save manually, type <code>&ltC-s&gt</code>. You can switch between the save buffers by entering the numbers <code>1-5</code>.
            </p>
`
        },
        {
            title: 'Edit Configuration',
            img: ImgConfig,
            body: `
            <p style="text-align: center; line-height:1.6em">
The configuration <code>wireman.toml</code> can be added in-app. On the services page, type <code>&ltC-e&gt</code>. Save the config with <code>&ltC-s&gt</code>. Exit with <code>&ltC-e&gt</code>.
            </p>
`
        },
        {
            title: 'Server Reflection',
            img: ImgReflection,
            body: `
<p style="text-align: center; line-height:1.6em">
Wireman supports server reflection. Enter <code>c-r</code>. Specify the server address and optional headers. Press <code>Enter</code> to send a reflection request. Exit reflection mode with <code>c-r</code>.
</p>
`
        }
    ];
    let num_elements = elements.length;

    let selected_index = $state(0);

    let startX = 0;
    let endX = 0;

    function handleTouchStart(event: TouchEvent) {
        startX = event.changedTouches[0].screenX;
    }

    function handleTouchEnd(event: TouchEvent) {
        endX = event.changedTouches[0].screenX;

        if (startX - endX > 50) {
            next();
        } else if (endX - startX > 50) {
            previous();
        }
    }

    function next() {
        if (selected_index + 1 >= num_elements) {
            return;
        } else {
            selected_index += 1;
        }
    }

    function previous() {
        if (selected_index == 0) {
            return;
        } else {
            selected_index -= 1;
        }
    }

    function handleKeydown(event: KeyboardEvent, action: 'next' | 'previous') {
        if (event.key === 'Enter' || event.key === ' ') {
            event.preventDefault();
            if (action === 'next') next();
            else previous();
        }
    }
</script>

{#snippet chevron(rotate: string, onclick: any, hide: boolean, action: 'next' | 'previous')}
    <button
        class="chevron-btn"
        {onclick}
        onkeydown={(e) => handleKeydown(e, action)}
        style="visibility: {hide ? 'hidden' : 'visible'};"
        aria-label={action === 'next' ? 'Next feature' : 'Previous feature'}
        tabindex={hide ? -1 : 0}
    >
        <svg class="chevron" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" style="transform: rotate({rotate});">
            <path d="M12 15L6 9H18L12 15Z" />
        </svg>
    </button>
{/snippet}

<div class="container" ontouchstart={handleTouchStart} ontouchend={handleTouchEnd}>
    <div class="navigation-dots">
        {#each elements as _, index}
            <button
                class="dot {index === selected_index ? 'active' : ''}"
                onclick={() => (selected_index = index)}
                aria-label={`Go to ${elements[index].title}`}
            ></button>
        {/each}
    </div>

    <div class="content-wrapper">
        <div class="title-section">
            {@render chevron('90deg', previous, selected_index == 0, 'previous')}
            <h3 class="feature-title">{elements[selected_index].title}</h3>
            {@render chevron('270deg', next, selected_index == num_elements - 1, 'next')}
        </div>

        <div class="body-section">
            <div class="description">
                {@html elements[selected_index].body}
            </div>

            <div class="img-container">
                <div class="img-wrapper">
                    <img class="feature-img" src={elements[selected_index].img} alt={elements[selected_index].title} loading="lazy" />
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .container {
        background-color: var(--gray-dark);
        border: 1px solid rgba(124, 139, 154, 0.2);
        border-radius: 12px;
        margin: 24px 16px 0 16px;
        padding: 32px 24px;
        position: relative;
        overflow: hidden;
        transition:
            border-color 0.3s ease,
            box-shadow 0.3s ease;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
    }

    .container::before {
        content: '';
        position: absolute;
        top: 0;
        left: 0;
        right: 0;
        height: 2px;
        background-color: var(--orange);
        opacity: 0.8;
    }

    .container:hover {
        border-color: rgba(124, 139, 154, 0.4);
        box-shadow: 0 6px 16px rgba(0, 0, 0, 0.2);
    }

    .navigation-dots {
        display: flex;
        justify-content: center;
        gap: 12px;
        margin-bottom: 24px;
    }

    .dot {
        all: unset;
        width: 10px;
        height: 10px;
        border-radius: 50%;
        background: rgba(255, 255, 255, 0.3);
        cursor: pointer;
        transition: all 0.2s ease;
        flex-shrink: 0;
        box-sizing: border-box;
    }

    .dot:hover {
        background: rgba(255, 255, 255, 0.5);
        transform: scale(1.2);
    }

    .dot.active {
        background: var(--orange);
        transform: scale(1.3);
    }

    .content-wrapper {
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .title-section {
        display: flex;
        justify-content: space-between;
        align-items: center;
        width: 100%;
        margin-bottom: 24px;
        padding: 0 16px;
    }

    .feature-title {
        padding: 0;
        margin: 0;
        color: var(--orange);
        font-size: var(--font-size-h2);
        font-weight: 600;
        text-align: center;
        flex: 1;
        letter-spacing: 0.5px;
    }

    .body-section {
        display: flex;
        flex-direction: column;
        align-items: center;
        width: 100%;
        gap: 24px;
    }

    .description {
        max-width: 600px;
        text-align: center;
    }

    .description :global(p) {
        margin: 0;
        font-size: calc(var(--font-size-body) + 1px);
        line-height: 1.6;
        color: rgba(255, 255, 255, 0.9);
    }

    .description :global(code) {
        font-family: var(--font-family-mono);
        background-color: var(--gray-darkest);
        color: var(--gray-lightest);
        border: 1px solid rgba(124, 139, 154, 0.3);
        border-radius: 4px;
        padding: 2px 6px;
        font-size: var(--font-size-code);
        font-weight: var(--font-weight-code);
    }

    .chevron-btn {
        background: rgba(255, 255, 255, 0.1);
        border: 1px solid rgba(255, 255, 255, 0.2);
        display: flex;
        justify-content: center;
        align-items: center;
        border-radius: 50%;
        width: 32px;
        height: 32px;
        cursor: pointer;
        transition: all 0.2s ease;
    }

    .chevron-btn:hover {
        background: rgba(255, 165, 0, 0.2);
        border-color: var(--orange);
        transform: scale(1.05);
    }

    .chevron-btn:active {
        transform: scale(0.95);
    }

    .chevron-btn:focus-visible {
        outline: 2px solid var(--orange);
        outline-offset: 2px;
    }

    .chevron {
        width: 16px;
        height: 16px;
        fill: var(--gray-lightest);
        transition: fill 0.2s ease;
    }

    .chevron-btn:hover .chevron {
        fill: var(--orange);
    }

    .img-container {
        width: 100%;
        max-width: 600px;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .img-wrapper {
        position: relative;
        width: 100%;
        border-radius: 12px;
        overflow: hidden;
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
        transition: box-shadow 0.3s ease;
    }

    .img-wrapper:hover {
        box-shadow: 0 12px 32px rgba(0, 0, 0, 0.4);
    }

    .feature-img {
        width: 100%;
        height: auto;
        display: block;
    }

    /* Mobile optimizations */
    @media (max-width: 600px) {
        .container {
            margin: 20px 10px 0 10px;
            padding: 24px 16px;
            border-radius: 8px;
        }

        .title-section {
            padding: 0 8px;
            margin-bottom: 20px;
        }

        .feature-title {
            font-size: var(--font-size-h3);
        }

        .chevron-btn {
            width: 28px;
            height: 28px;
        }

        .chevron {
            width: 14px;
            height: 14px;
        }

        .body-section {
            gap: 20px;
        }

        .description :global(p) {
            font-size: var(--font-size-body);
        }

        .navigation-dots {
            gap: 10px;
            margin-bottom: 20px;
        }

        .dot {
            width: 8px;
            height: 8px;
        }
    }

    @media (max-width: 800px) {
        .img-container {
            max-width: 100%;
        }
    }

    /* Reduced motion support */
    @media (prefers-reduced-motion: reduce) {
        .container,
        .chevron-btn,
        .dot,
        .img-wrapper {
            transition: none;
        }
    }
</style>
