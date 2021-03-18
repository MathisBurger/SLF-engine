import React from 'react';
import style from "../styles/Home.module.css";

export default function TableComponent(props) {
    if (props.render) {
        let list = [];
        for (let i=0; i<props.data.length; i++) {
            list.push(
              <tr>
                  <td className={style.table}>{props.data[i].name}</td>
                  <td className={style.table}>{props.data[i].description}</td>
                  <td className={style.table}><a href={props.data[i].source}>{props.data[i].source}</a></td>
              </tr>
            );
        }
        return (
            <table className={style.table}>
                <thead>
                <tr>
                    <th>Value</th>
                    <th>Description</th>
                    <th>Source</th>
                </tr>
                </thead>
                <tbody>
                {list}
                </tbody>
            </table>
        );
    } else {
        return null;
    }
}
