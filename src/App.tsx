import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

import BookBtn from "./assets/components/book-btn";

function App() {
  const [books, setBooks] = useState<Array<Record<string, any>>>([]);

  useEffect(() => {
    let getBooks = async () => { 
        let result: Array<any> = await invoke("get_books");
        setBooks(JSON.parse(result));
    }
    getBooks(); 
  }, []);

  return (
    <div className="page-splash flex flex-col h-screen items-center justify-center pb-40">
      <h1 className="text-indigo-400 font-thin text-5xl tracking-wide mb-3">INBAN</h1>
      <h2 className="text-indigo-300 text-s font-bold">Keeping your wallet thicc(er)</h2>

      <div className="bg-indigo-800 border-indigo-700 border-solid border-2 mt-10 rounded-lg p-5 w-[400px]">
        <ul className="text-indigo-200">
            { 
                books.map(book => (
                    <BookBtn id={book.id} name={book.name} />
                ))
            }
        </ul>
      </div>
    </div>
  );
}

export default App;
