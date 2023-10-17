export default function BookBtn(props: { name: string }) {
    return (
            <li><a href="#" className="block button book-btn transition-colors focus:bg-indigo-700 border-2 border-transparent hover:border-indigo-700 px-3 py-2 mb-1 mt-1 rounded-lg">{props.name}</a></li>
   );
}
