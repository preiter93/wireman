<script lang="ts">
    import ImgResponse from '$lib/assets/website_271.png';
    import ImgConfig from '$lib/assets/website_439.png';
    import ImgReflection from '$lib/assets/website_476.png';

    type ShowElement = {
        title: string;
        img: string;
        // TODO: Use snippets
        body: string;
    };

    let elements: ShowElement[] = [
        {
            title: 'Save history',
            img: ImgResponse,
            body: `
            <p style="text-align: center; line-height:1.4em">
Sent requests are automatically saved. To save manually, type <code>&ltC-s&gt</code>. You can switch between the save buffers by entering the numbers <code>1-5</code>.
            </p>
`
        },
        {
            title: 'Edit Configuration',
            img: ImgConfig,
            body: `
            <p style="text-align: center; line-height:1.4em">
The configuration <code>wireman.toml</code> can be added in-app. On the services page, type <code>&ltC-e&gt</code>. Save the config with <code>&ltC-s&gt</code>. Exit with <code>&ltC-e&gt</code>. 
            </p>
`
        },
        {
            title: 'Server Reflection',
            img: ImgReflection,
            body: `
<p style="text-align: center; line-height:1.4em">
Wireman supports server reflection. Enter <code>c-r</code>. Specify the server address and optional headers. Press <code>Enter</code> to send a reflection request. If reflection is successful, the services and the method list are updated. Exit reflection mode with <code>c-r</code>.
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
</script>

{#snippet chevron(rotate: string, onclick: any, hide: boolean)}
    <button class="chevron-btn" {onclick} style="visibility: {hide ? 'hidden' : 'visible'};">
        <svg class="chevron" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" style="transform: rotate({rotate});">
            <path class="chevron path" d="M12 15L6 9H18L12 15Z" />
            >
        </svg>
    </button>
{/snippet}

<div class="container" ontouchstart={handleTouchStart} ontouchend={handleTouchEnd}>
    <div class="title">
        {@render chevron('90deg', previous, selected_index == 0)}
        <h3>{elements[selected_index].title}</h3>
        {@render chevron('270deg', next, selected_index == num_elements - 1)}
    </div>
    <div class="body">
        {@html elements[selected_index].body}

        <div class="img-container">
            <img class="img" src={elements[selected_index].img} alt={elements[selected_index].title} />
        </div>
    </div>
</div>

<style>
    .container {
        padding-top: 12px;
        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;
        background-color: var(--gray-dark);
        border-radius: 8px;
        margin: 20px 12px 0px 12px;
        padding: 16px 20px 16px 20px;
    }

    h3 {
        padding: 0;
        margin: 0;
        color: var(--orange);
        font-size: var(--font-size-h5);
        font-weight: var(--font-weight-body);
    }

    .title {
        display: flex;
        justify-content: space-between;
        align-items: center;
        width: 100%;
        margin-bottom: 1rem;
        padding: 0px 16px 0px 16px;
    }

    .body {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding: 0px 8px 0px 8px;
    }

    @media (max-width: 600px) {
        .container {
            margin: 16px 10px 0px 10px;
            padding: 10px 12px 10px 12px;
        }
        .title {
            padding: 0px 6px 0px 6px;
        }
        .body {
            padding: 0px 14px 0px 14px;
        }
    }

    .chevron-btn {
        background-color: transparent;
        color: transparent;
        display: flex;
        justify-content: center;
        align-items: center;
        fill: var(--gray-lightest);
        box-sizing: border-box;
        border: none;
        box-shadow: 0px 0px 2px var(--gray-lightest);
        border-radius: 50%;
        height: var(--icon-size);
        width: var(--icon-size);
        margin: 0;
        padding: 0;
    }

    .chevron-btn:hover {
        transform: scale(0.9);
    }
    .chevron {
        height: 100%;
        width: 100%;
        transform: translateY(1px);
    }

    .img-container {
        padding-top: 16px;
        display: flex;
        justify-content: center;
        align-items: center;
        width: 600px;
    }

    .img {
        height: auto;
        display: block;
        margin: 0;
        width: 600px;
        box-shadow: 0px 0px 3px var(--gray-light);
    }

    @media (max-width: 800px) {
        .img-container {
            width: 100%;
        }
        .img {
            width: 100%;
        }
    }
</style>
