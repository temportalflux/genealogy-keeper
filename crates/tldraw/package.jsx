import * as React from 'react'
import { createRoot } from 'react-dom/client'
import * as tldraw from 'tldraw'
import 'tldraw/tldraw.css'

export * as tldraw from 'tldraw'

export function createElementWithin_Tldraw(dom_node, props) {
	return createRoot(dom_node).render(<tldraw.Tldraw {...props} />);
}
