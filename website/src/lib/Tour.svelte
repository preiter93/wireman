<script lang="ts">
    import ImgHelp from '$lib/assets/website_001.png';
    import ImgServices from '$lib/assets/website_026.png';
    import ImgMethod from '$lib/assets/website_080.png';
    import ImgAuth from '$lib/assets/website_170.png';
    import ImgRequest from '$lib/assets/website_225.png';
    import ImgCopy from '$lib/assets/website_326.png';

    type ShowElement = {
        title: string;
        img: string;
        // TODO: Use snippets
        body: string;
    };

    let elements: ShowElement[] = [
        {
            title: 'Open help',
            img: ImgHelp,
            body: `
            <p style="text-align: center; line-height:1.6em">
                Open the help dialog by pressing <code>?</code>. It provides key mappings for each page to help you navigate.
            </p>
`
        },
        {
            title: 'Select service',
            img: ImgServices,
            body: `
            <p style="text-align: center; line-height:1.6em">
                Get started by selecting a proto service. Use <code>j/k</code> to scroll and
                <code>Enter</code> to select a service.
            </p>
`
        },
        {
            title: 'Select method',
            img: ImgMethod,
            body: `
            <p style="text-align: center; line-height:1.6em">
                Use <code>j/k</code> to scroll through the methods and
                click <code>Enter</code> to select a method and progress.
            </p>
`
        },
        {
            title: 'Set address and headers',
            img: ImgAuth,
            body: `
            <p style="text-align: center; line-height:1.6em">
                Edit the server address and headers. Insert with <code>i</code> and go to normal mode with <code>Esc</code>.
                Add headers with <code>&ltC-a&gt</code>.
                Use scripts for auth headers by enclosing them in <code>$()</code>. Omit the "Bearer" or "Basic" prefixes in the auth headers.
                Press <code>Tab</code> to continue.
            </p>
`
        },
        {
            title: 'Send request',
            img: ImgRequest,
            body: `
            <p style="text-align: center; line-height:1.6em">
                Edit the request. Navigate with <code>h/j/k/l</code>, insert with <code>i</code> and go to normal mode with <code>Esc</code>. For more mappings visit <a href="https://github.com/preiter93/edtui" class="custom-link" target="_blank" rel="noopener noreferrer">edtui</a>. Press <code>Enter</code> to send the request.
            </p>
`
        },
        {
            title: 'Copy Response',
            img: ImgCopy,
            body: `
            <p style="text-align: center; line-height:1.6em">
                Switch between the request and response editors using <code>J/K</code>. Copy the response with <code>VGy</code> to your clipboard.
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
        aria-label={action === 'next' ? 'Next step' : 'Previous step'}
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
            <h3 class="step-title">{elements[selected_index].title}</h3>
            {@render chevron('270deg', next, selected_index == num_elements - 1, 'next')}
        </div>

        <div class="body-section">
            <div class="description">
                {@html elements[selected_index].body}
            </div>

            <div class="img-container">
                <div class="img-wrapper">
                    <img class="step-img" src={elements[selected_index].img} alt={elements[selected_index].title} loading="lazy" />
                </div>
            </div>
        </div>
    </div>
</div>

<style>
    .container {
        background-color: var(--gray-dark);
        border: 1px solid rgba(124, 139, 154, 0.2);
        border-radius: var(--border-radius);
        margin: var(--card-margin);
        padding: var(--card-padding);
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
        height: 1px;
        background-color: var(--orange);
        opacity: 0.4;
    }

    .container:hover {
        border-color: rgba(124, 139, 154, 0.4);
        box-shadow: 0 6px 16px rgba(0, 0, 0, 0.2);
    }

    .navigation-dots {
        display: flex;
        justify-content: center;
        gap: var(--dot-gap);
        margin-bottom: var(--content-spacing);
    }

    .dot {
        all: unset;
        width: var(--dot-size);
        height: var(--dot-size);
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
        margin-bottom: var(--content-spacing);
        padding: 0 16px;
    }

    .step-title {
        padding: 0;
        margin: 0;
        color: var(--orange);
        font-size: var(--font-size-h3);
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
        gap: var(--content-spacing);
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
        width: var(--chevron-btn-size);
        height: var(--chevron-btn-size);
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
        width: var(--chevron-size);
        height: var(--chevron-size);
        fill: var(--gray-lightest);
        transition: fill 0.2s ease;
    }

    .chevron-btn:hover .chevron {
        fill: var(--orange);
    }

    .img-container {
        width: 100%;
        max-width: var(--img-max-width);
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .img-wrapper {
        position: relative;
        width: 100%;
        border-radius: var(--border-radius);
        overflow: hidden;
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
        transition: box-shadow 0.3s ease;
    }

    .img-wrapper:hover {
        box-shadow: 0 12px 32px rgba(0, 0, 0, 0.4);
    }

    .step-img {
        width: 100%;
        height: auto;
        display: block;
    }

    /* Mobile optimizations */
    @media (max-width: 600px) {
        .title-section {
            padding: 0 8px;
        }

        .step-title {
            font-size: var(--font-size-h3);
        }

        .description :global(p) {
            font-size: var(--font-size-body);
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
