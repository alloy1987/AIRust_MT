// 移植自 editor/muya/src/types/index.d.ts 的静态资源与无类型依赖垫片，
// 供 vue-tsc 检查 @muyajs/core 源码图时使用（该文件不属于本项目 tsconfig 范围）。
declare module '*.svg';
declare module '*.png';
declare module '*.jpg';
declare module '*.jpeg';
declare module '*.gif';
declare module '*.bmp';
declare module '*.tiff';
declare module '*.css';
declare module '*.css?inline';
declare module '*.woff';
declare module '*.woff2';
declare module '*.woff2?inline' {
    const dataUri: string;
    export default dataUri;
}
declare module 'joplin-turndown-plugin-gfm';
declare module 'prismjs' {
    export interface Token {
        type: string;
        content: string | Token[] | Token;
        alias?: boolean | string[];
        length: number;
        toString(): string;
    }
    export const languages: Record<string, any>;
    const prism: {
        languages: Record<string, any>;
        highlight(code: string, grammar?: unknown, language?: string): string;
        tokenize(code: string, grammar?: unknown): (string | Token)[];
        highlightElement(
            element: Element,
            async?: boolean,
            callback?: (this: HTMLElement) => void,
        ): void;
        Token?: typeof import('prismjs').Token;
    };
    export default prism;
}
declare module 'prismjs/components.js' {
    export const languages: Record<string, any>;
    const components: { languages: Record<string, any>; [key: string]: unknown };
    export default components;
}
declare module 'prismjs/dependencies' {
    export default function getLoader(
        components: { languages: Record<string, any> },
        langs: string[],
        loaded: Iterable<string>,
    ): {
        load(
            callback: (lang: string, chain?: unknown) => unknown,
            options?: {
                series?: (before: Promise<void>, after: () => Promise<void>) => Promise<void>;
                parallel?: (values: Promise<void>[]) => Promise<void[]>;
            },
        ): Promise<void>;
    };
}
declare module 'prismjs/plugins/keep-markup/prism-keep-markup';
declare module 'plantuml-encoder' {
    export function encode(data: string): string;
    export function decode(data: string): string;
}
declare module 'turndown' {
    export type Filter = string | string[] | ((node: Node) => boolean);
    export interface Node {
        nodeType: number;
        nodeName: string;
        parentElement: Node | null;
        parentNode: Node | null;
        nextSibling: Node | null;
        textContent?: string | null;
        style?: CSSStyleDeclaration;
        cloneNode?(): Node;
        replaceWith?(other: Node): void;
    }
    export class TurndownService {
        constructor(options?: TurndownService.Options);
        options: TurndownService.Options;
        escape: (s: string) => string;
        addRule(key: string, rule: TurndownService.Rule): this;
        remove(filter: Filter): this;
        keep(filter: Filter): this;
        use(plugin: (service: TurndownService) => void): this;
        turndown(html: string | Node): string;
    }
    export namespace TurndownService {
        interface Options {
            headingStyle?: string;
            hr?: string;
            bulletListMarker?: string;
            codeBlockStyle?: string;
            fence?: string;
            emDelimiter?: string;
            strongDelimiter?: string;
            linkStyle?: string;
            linkReferenceStyle?: string;
            blankLines?: string;
        }
        interface Rule {
            filter?: Filter | { condition: (node: Node) => boolean };
            replacement: (content: string, node: Node, options: Options) => string;
        }
    }
    export { TurndownService as default };
}
declare module '@marktext/file-icons';
declare module 'snapsvg-cjs';
declare module 'vega-embed';
declare module 'vega-lite';

interface Window {
    Prism: unknown;
    DIRNAME?: string;
    MUYA_VERSION?: string;
}

declare const process: { env: Record<string, string | undefined> };

interface Element {
    __MUYA_BLOCK__?: { blockName: string } | null;
}

declare module 'flowchart.js' {
    interface IFlowChartDrawOptions {
        [key: string]: unknown;
    }
    interface IFlowChartInstance {
        drawSVG: (container: HTMLElement | string, options?: IFlowChartDrawOptions) => void;
        clean: () => void;
    }
    export function parse(input: string): IFlowChartInstance;
    const flowchart: { parse: typeof parse };
    export default flowchart;
}

declare module '*sequence-diagram-snap' {
    interface ISequenceDrawOptions {
        theme?: 'hand' | 'simple';
        [key: string]: unknown;
    }
    interface ISequenceDiagramInstance {
        drawSVG: (container: HTMLElement | string, options?: ISequenceDrawOptions) => void;
    }
    interface ISequenceDiagramConstructor {
        parse: (input: string) => ISequenceDiagramInstance;
    }
    const Diagram: ISequenceDiagramConstructor;
    export default Diagram;
}