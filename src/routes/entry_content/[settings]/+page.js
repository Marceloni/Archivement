/** @type {import('./$types').PageLoad} */
export function load({ params }) {
	return {settings: params.settings};
}