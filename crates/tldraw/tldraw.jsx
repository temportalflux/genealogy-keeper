import * as React from 'react'
import { createRoot } from 'react-dom/client'
import * as tldraw from 'tldraw'
import 'tldraw/tldraw.css'

function createElementWithin(dom_node, element) {
	return createRoot(dom_node).render(element);
}

/**
	Create the [Tldraw](https://tldraw.dev/docs/editor) react component within the provided DOM node.

	[Tldraw Source](https://github.com/tldraw/tldraw/blob/v3.12.1/packages/tldraw/src/lib/Tldraw.tsx#L81)
	
	@param {Element} dom_node The DOM element to construct the react component as a child of.
	@param {TldrawProps} props The [properties](https://tldraw.dev/reference/tldraw/Tldraw) of the editor.
*/
export var createElementWithin_Tldraw = (dom_node, props) => createElementWithin(dom_node, <tldraw.Tldraw {...props} />);
