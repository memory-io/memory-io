

import type { StudySet } from '$lib/types';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
    const response = (await fetch(`/api/sets/recents`));

    const recent_sets: StudySet[] = await response.json();
	return {
		recent_sets
	};
};