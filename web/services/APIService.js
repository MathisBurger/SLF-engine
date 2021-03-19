
const BASE_URL = 'http://127.0.0.1:8080/api';

// Fetch data from the API.
// only returns response of /api/fetch endpoint
export async function fetchData(category, letter) {
    const res = await window.fetch(`${BASE_URL}/fetch?category=${category}&letter=${letter}`, {
        method: 'GET',
        mode: 'cors'
    });
    return await res.json();
}

// calls the API to check if given category
// is a synonym for a present category
export async function parseSynonym(category) {
    const res = await window.fetch(`${BASE_URL}/parse_synonym?synonym=${category}`, {
        method: 'GET',
        mode: 'cors'
    });
    return await res.json();
}
