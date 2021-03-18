import Header from '../components/Header';
import style from '../styles/Home.module.css';
import {useState} from 'react';
import {search} from "../services/SearchService";

export default function Home() {

  const [text, changeText] = useState("");

  return (
    <div>
        <Header />
      <div className={style.container}>
              <input className={style.searchbar}
                     placeholder={"city with a"}
                     onChange={event => changeText(event.target.value)}
                     onKeyPress={(event) => onSubmit(event, text)}
              />
      </div>
    </div>
  )
}

async function onSubmit(event, text) {
    if (event.key === 'Enter') {
        let data = await search(text);
        if (!data.status) {
            window.alert(data.message);
        }
    }
}
