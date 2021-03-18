import Header from '../components/Header';
import TableComponent from "../components/TableComponent";

import style from '../styles/Home.module.css';
import {useState} from 'react';
import {search} from "../services/SearchService";

export default function Home() {

  const [text, changeText] = useState("");
  const [renderTable, changeRenderTable] = useState(false);
  const [tableData, changeTableData] = useState([]);

  return (
    <div>
        <Header />
      <div className={style.container}>
              <input className={style.searchbar}
                     placeholder={"city with a"}
                     onChange={event => changeText(event.target.value)}
                     onKeyPress={(event) => onSubmit(event, text, changeRenderTable, changeTableData)}
              />
            <TableComponent render={renderTable} data={tableData} />
      </div>
    </div>
  )
}

async function onSubmit(event, text, renderChanger, contentChanger) {
    if (event.key === 'Enter') {
        let data = await search(text);
        if (!data.status) {
            window.alert(data.message);
        } else {
            renderChanger(true);
            contentChanger(data.data);
        }
    }
}
