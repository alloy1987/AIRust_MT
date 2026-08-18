import type { Muya } from '../../../muya';
import type { IFrontmatterState, TState } from '../../../state/types';
import { parseDocument } from 'yaml';
import { fromEvent } from 'rxjs';
import { escapeHTML } from '../../../utils';
import logger from '../../../utils/logger';
import Parent from '../../base/parent';

const debug = logger('frontmatterPreview:');

// Display semantics mirror Markpad's frontmatter panel: lists join with
// commas, dates fall back to ISO strings, nested objects render as JSON.
function stringifyDisplayValue(value: unknown): string {
    if (Array.isArray(value))
        return value.map(item => stringifyDisplayValue(item)).join(', ');

    if (value === null || value === undefined)
        return '';

    if (value instanceof Date)
        return value.toISOString();

    if (typeof value === 'object')
        return JSON.stringify(value);

    return String(value);
}

// Front matter is document metadata, so it must parse as a YAML mapping to
// earn the key/value panel; anything else (scalar, sequence) falls back to
// the raw-text view.
function isMapping(value: unknown): boolean {
    return value !== null
        && typeof value === 'object'
        && !Array.isArray(value)
        && !(value instanceof Date);
}

class FrontmatterPreview extends Parent {
    private _text: string;
    private readonly _lang: string;

    static override blockName = 'frontmatter-preview';

    static create(muya: Muya, state: IFrontmatterState) {
        return new FrontmatterPreview(muya, state);
    }

    override get path() {
        debug.warn('You can never call `get path` in frontmatterPreview');
        return [];
    }

    constructor(muya: Muya, { text, meta }: IFrontmatterState) {
        super(muya);
        this.tagName = 'div';
        this._text = text;
        this._lang = meta.lang;
        this.classList = ['mu-frontmatter-preview'];
        this.attributes = {
            spellcheck: 'false',
            contenteditable: 'false',
        };
        this.createDomNode();
        this._attachDOMEvents();
        this.update();
    }

    override getState(): TState {
        debug.warn('You can never call `getState` in frontmatterPreview');
        return {} as TState;
    }

    private _attachDOMEvents() {
        const clickObservable = fromEvent(this.domNode!, 'click');
        clickObservable.subscribe(this.clickHandler.bind(this));
    }

    clickHandler(event: Event) {
        event.preventDefault();
        event.stopPropagation();

        const cursorBlock = this.parent!.firstContentInDescendant();
        cursorBlock?.setCursor(0, 0);
    }

    update(text = this._text) {
        if (this._text !== text)
            this._text = text;

        // Only YAML (`---`) fences are parsed into a key/value panel; the
        // TOML (`+++`) and JSON (`;;;` / `{}`) styles have no bundled parser
        // and keep the raw-text view.
        if (this._lang !== 'yaml') {
            this._renderRaw(text);
            return;
        }

        const doc = parseDocument(text, { prettyErrors: false });
        if (doc.errors.length > 0) {
            const { i18n } = this.muya;
            const message = doc.errors.map(error => error.message).join('\n');
            this.domNode!.innerHTML
                = `<div class="mu-frontmatter-error">${escapeHTML(i18n.t('Invalid Front Matter'))}\n${escapeHTML(message)}</div>`;
            return;
        }

        const parsed = doc.toJSON();
        if (parsed === null || parsed === undefined) {
            this.domNode!.innerHTML = this._renderHeader(0);
            return;
        }

        if (!isMapping(parsed)) {
            this._renderRaw(text);
            return;
        }

        const entries = Object.entries(parsed as Record<string, unknown>);
        const rows = entries
            .map(([key, value]) =>
                `<div class="mu-frontmatter-key">${escapeHTML(key)}</div>`
                + `<div class="mu-frontmatter-value">${escapeHTML(stringifyDisplayValue(value))}</div>`)
            .join('');

        this.domNode!.innerHTML = this._renderHeader(entries.length)
            + (rows ? `<div class="mu-frontmatter-grid">${rows}</div>` : '');
    }

    private _renderHeader(count: number) {
        const { i18n } = this.muya;

        return `<div class="mu-frontmatter-header">`
            + `<span class="mu-frontmatter-title">${escapeHTML(i18n.t('Front Matter'))}</span>`
            + `<span class="mu-frontmatter-count">${count}</span>`
            + `</div>`;
    }

    private _renderRaw(text: string) {
        this.domNode!.innerHTML
            = `<pre class="mu-frontmatter-raw">${escapeHTML(text)}</pre>`;
    }
}

export default FrontmatterPreview;
