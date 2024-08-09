use crate::t;
use leptos::*;

#[component]
pub fn Credits(
    set_show_credits: WriteSignal<bool>,
) -> impl IntoView {
    let show_book_link = std::env!("PLATFORM") != "STEAM";

    view! {
        <div class="credits">
            <div class="credits--inner">
                <h2>{t!("Concept")}</h2>
                <ul>
                    <li>Arthur Röing Baer</li>
                    <li>Chiara Di Leone</li>
                    <li>Drew Pendergrass</li>
                    <li>Son La Pham</li>
                    <li>Francis Tseng</li>
                    <li>Gregory Vettese</li>
                    <li>Troy Vettese</li>
                </ul>
                <h2>{t!("Design")}</h2>
                <ul>
                    <li>Son La Pham</li>
                    <li>Francis Tseng</li>
                </ul>
                <h2>{t!("Development")}</h2>
                <ul>
                    <li>Francis Tseng</li>
                    <li>Son La Pham</li>
                </ul>
                <h2>{t!("Research and Writing")}</h2>
                <ul>
                    <li>Lucy Chinen</li>
                    <li>Drew Pendergrass</li>
                    <li>Son La Pham</li>
                    <li>Spencer Roberts</li>
                    <li>Justin Saint-Loubert-Bié</li>
                    <li>Francis Tseng</li>
                    <li>Troy Vettese</li>
                </ul>
                <h2>{t!("Music")}</h2>
                <ul>
                    <li>PRINCE SHIMA</li>
                </ul>
                <h2>{t!("Playtesting")}</h2>
                <ul>
                    <li>Spencer Roberts</li>
                    <li>Dargan Frierson</li>
                    <li>Sean Raspet</li>
                    <li>Sarah Friend</li>
                    <li>Filip Mesko</li>
                    <li>Wassim Alsindi</li>
                    <li>Bradley K</li>
                    <li>Julia</li>
                    <li>Grace Van de Pas</li>
                    <li>Michael Vettese</li>
                    <li>Xinyue</li>
                    <li>Lukas Eigler-Harding</li>
                    <li>Adrian Dinh</li>
                    <li>Aural Ephem</li>
                    <li>Nick Houde</li>
                    <li>Simon Zhang</li>
                    <li>Paul Turberg</li>
                    <li>Jan Philipp Dapprich</li>
                    <li>Matt Goerzen</li>
                    <li>Neilson Koerner-Safrata</li>
                    <li>Kira Simon-Kennedy</li>
                    <li>Nicholas Carter</li>
                </ul>
                <h2>{t!("Translation")}</h2>
                <ul>
                    <li>Leo "Fujoneko" Belo</li>
                    <li>Eduardo Eloy</li>
                    <li>Marco Mangan</li>
                    <li>Francisco Jota-Pérez</li>
                    <li>Víctor Anadón Vega</li>
                    <li>Christoph Rupprecht</li>
                    <li>Thomas Helmis</li>
                    <li>Merlin B.</li>
                    <li>Chayangoon Thamma-Un, Thai translator</li>
                    <li>
                        เนติวิทย์ โชติภัทร์ไพศาล / Netiwit Chotiphatphaisal, Sam Yan Press
                    </li>
                    <li>Fatih Erdoğan</li>
                </ul>
                <Show
                    when=move || { show_book_link }
                    fallback=|| {
                        view! {
                            <h3>
                                {t!("Based on the book")} <em>Half-Earth Socialism</em>
                                (Drew Pendergrass &amp; Troy Vettese, Verso 2022).
                            </h3>
                        }
                    }
                >
                    <h3>
                        {t!("Based on the book")} <em>
                            <a href="https://www.versobooks.com/books/3818-half-earth-socialism">
                                Half-Earth Socialism
                            </a>
                        </em> (Drew Pendergrass &amp; Troy Vettese, Verso 2022).
                    </h3>
                </Show>
                <h2>{t!("Thank you for playing!")}</h2>

                <button
                    class="btn"
                    on:click=move |_| {
                        set_show_credits.set(false);
                    }
                >
                    {t!("Back")}
                </button>
            </div>
        </div>
    }
}
