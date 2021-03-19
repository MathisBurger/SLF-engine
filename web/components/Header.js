import styles from '../styles/Header.module.css';
import {faGithub} from "@fortawesome/free-brands-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";


// This is the Header component
// It displays the header of the webpage
export default function Header() {
    return (
        <div className={styles.container}>
            <div className={styles.heading}>SLF-engine</div>
            <div className={styles.linkBox}>
                <a href={"https://github.com/MathisBurger/SLF-engine"}>
                    <FontAwesomeIcon icon={faGithub} size={"5x"} />
                </a>
            </div>
        </div>
    );
}
