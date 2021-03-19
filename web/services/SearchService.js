import {fetchData, parseSynonym} from "./APIService";

// main search function
// It handles the text length and
// data fetching
export async function search(text) {
    let split = text.split(" ");
    if (split.length === 2) {
        return await getData(split[0], split[1]);
    } else if (split.length > 2) {
        return await getData(split[0], split[(split.length - 1)]);
    } else {
        return {status: false, message: "Not enough search words"};
    }
}

// handles category and synonym parsing
async function getData(category, letter) {
    let data = await fetchData(category, letter);
    if (!data.status) {
        let syn = await parseSynonym(category);
        if (syn.status) {
            return await fetchData(syn.category, letter);
        } else {
            return data;
        }
    } else {
        return data;
    }
}
