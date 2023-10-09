import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

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
    <div className="flex flex-col h-screen items-center justify-center">
      <h1 className="text-indigo-400 font-thin text-5xl tracking-wide mb-3">INBAN</h1>

      <main className="bg-indigo-800 border-indigo-700 border-solid border-2 mt-10 rounded-lg p-5 w-[400px]">
        <ul className="text-indigo-200">
            { 
                books.map(book => (
                    <li key="{book.id}">{ book.name }</li>
                ))
            }
        </ul>
      </main>
    </div>
  );
}

export default App;
