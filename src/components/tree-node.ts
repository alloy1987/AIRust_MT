import type { TreeEntry } from '../api'

export interface TreeNode {
  key: string
  entry: TreeEntry
  expanded: boolean
  loading: boolean
  /** null = 尚未加载（懒加载：首次展开时才读取子项） */
  children: TreeNode[] | null
}

export function toNode(entry: TreeEntry): TreeNode {
  return { key: entry.path, entry, expanded: false, loading: false, children: null }
}