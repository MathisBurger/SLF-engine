import Header from '../components/Header';
import style from '../styles/Home.module.css';


export default function Home() {
  return (
    <div>
        <Header />
      <div className={style.container}>
          <input className={style.searchbar} placeholder={"city with a"} />
      </div>
    </div>
  )
}
