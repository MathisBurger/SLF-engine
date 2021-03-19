
const BASE_URL = 'http://127.0.0.1:8080/api';

export async function fetchData(category, letter) {
    const res = await window.fetch(`${BASE_URL}/fetch?category=${category}&letter=${letter}`, {
        method: 'GET',
        mode: 'cors'
    });
    return await res.json();
}

export async function parseSynonym(category) {
    const res = await window.fetch(`${BASE_URL}/parse_synonym?synonym=${category}`, {
        method: 'GET',
        mode: 'cors'
    });
    return await res.json();
}
