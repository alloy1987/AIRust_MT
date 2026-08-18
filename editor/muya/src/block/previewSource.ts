import type Content from './base/content';

// Blocks that flip from a rendered preview to their raw-source editor when
// they receive the caret (see the `figure.mu-active.mu-*-block` and
// `pre.mu-active.mu-frontmatter` rules in blockSyntax.css).
const PREVIEW_SOURCE_BLOCKS = /^(?:(?:html|math|diagram)-block|frontmatter)$/;

export function isInsidePreviewSourceBlock(block: Content): boolean {
    return block.getAncestors().some(a => PREVIEW_SOURCE_BLOCKS.test(a.blockName));
}

// Walk backwards / forwards past preview-source blocks so an auto-placed
// caret lands in a block that stays in its normal view.
export function skipPreviewSourceBackward(content: Content | null): Content | null {
    let c = content;
    while (c && isInsidePreviewSourceBlock(c))
        c = c.previousContentInContext() ?? null;

    return c;
}

export function skipPreviewSourceForward(content: Content | null): Content | null {
    let c = content;
    while (c && isInsidePreviewSourceBlock(c))
        c = c.nextContentInContext() ?? null;

    return c;
}
