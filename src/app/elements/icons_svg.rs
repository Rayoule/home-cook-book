use leptos::prelude::*;

/// Main Page Logo
#[component]
pub fn LogoSVG() -> impl IntoView {
    view! {
        <img src="/assets/logo.svg" class="logo-img" />
    }
}

#[component]
pub fn SearchSVG() -> impl IntoView {
    view! {
        <svg
            class="search-icon-svg"
            viewBox="0 0 100 100"
            stroke="black"
            stroke-width="3"
        >
            <circle
                cx="50"
                cy="50"
                r="40"
            />
        </svg>
    }
}


#[component]
pub fn SortUpDownVG(is_up: bool) -> impl IntoView {
    let class = "sort-up-down-icon-svg ".to_string() + if is_up { "up" } else { "down" };

    view! {
        <svg
            class=class
            viewBox="0 0 24 24"
            fill="none"
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M16 14L12 10L8 14"
            />
        </svg>
    }
}

#[component]
pub fn SortSVG() -> impl IntoView {
    view! {
        <svg
            class="sort-icon-svg"
            viewBox="0 0 16 16"
        >
            <path
                d="M3.25 1A2.25 2.25 0 001 3.25v9.5A2.25 2.25 0 003.25 15h9.5A2.25 2.25
                0 0015 12.75v-9.5A2.25 2.25 0 0012.75 1h-9.5z"
            />
        </svg>
    }
}

#[component]
pub fn RemoveSVG() -> impl IntoView {
    view! {
        <svg
            class="remove-icon-svg"
            viewBox="0 0 100 100"
        >
            <circle
                cx="50"
                cy="50"
                r="40"
            />
        </svg>
    }
}

#[component]
pub fn BackButtonSVG(#[prop(optional)] backup_page: bool) -> impl IntoView {
    let backup_page_class = if backup_page {
        " backup-page".to_string()
    } else {
        "".to_string()
    };

    view! {

        <svg
            class="back-icon-svg".to_string() + &backup_page_class
            viewBox="0 0 25.658487 35.611439"
            xml:space="preserve"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:svg="http://www.w3.org/2000/svg"
        >
            <g transform="translate(-21.938443,-152.32135)" >
                <path
                    d="m 47.596927,155.68283 2e-6,28.88849 a 3.3614676,3.3614676 152.96614 0 1 -5.334073,2.72182 L 23.91105,173.99289 a 
                    4.7743054,4.7743054 89.999993 0 1 -1e-6,-7.73162 l 18.351806,-13.30026 a 3.3614669,3.3614669 27.03385 0 1 
                    5.334072,2.72182 z"
                />
            </g>
        </svg>

    }
}

#[component]
pub fn PrintButtonSVG(color: String) -> impl IntoView {
    view! {

        <svg
            class="recipe-menu-icon print"
            fill=color
            viewBox="0 0 31.63673 37.23069"
            xml:space="preserve"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:svg="http://www.w3.org/2000/svg"
        >
            <g transform="translate(-7.4741726,-221.84271)" >
                <g transform="matrix(1.8615344,0,0,1.8615344,1.0956177,218.07705)" >
                    <path
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        d="m 4.598074,3.1944521 c -1.17157,1.17158 -1.17157,3.05719 -1.17157,6.8284299 v 4 c 0,3.7712 0,5.6569 1.17157,6.8284 1.17158,1.1716 3.05719,1.1716 6.82843,1.1716 h 0.994976 c 3.7712,0 5.6569,0 6.8284,-1.1716 1.1716,-1.1715 1.1716,-3.0572 1.1716,-6.8284 v -4 c 0,-3.7712399 0,-5.6568499 -1.1716,-6.8284299 -1.1715,-1.17157 -3.0572,-1.17157 -6.8284,-1.17157 h -0.994976 c -3.77124,0 -5.65685,0 -6.82843,1.17157 z m 2.575918,4.82843 c 0,-0.41421 0.33579,-0.75 0.75,-0.75 h 7.497488 c 0.4142,0 0.75,0.33579 0.75,0.75 0,0.41421 -0.3358,0.75 -0.75,0.75 H 7.923992 c -0.41421,0 -0.75,-0.33579 -0.75,-0.75 z m 0,3.9999999 c 0,-0.4142 0.33579,-0.75 0.75,-0.75 h 7.497488 c 0.4142,0 0.75,0.3358 0.75,0.75 0,0.4142 -0.3358,0.75 -0.75,0.75 H 7.923992 c -0.41421,0 -0.75,-0.3358 -0.75,-0.75 z m 0.75,3.25 c -0.41421,0 -0.75,0.3358 -0.75,0.75 0,0.4142 0.33579,0.75 0.75,0.75 h 4.497488 c 0.4142,0 0.75,-0.3358 0.75,-0.75 0,-0.4142 -0.3358,-0.75 -0.75,-0.75 z"
                    />
                </g>
            </g>
        </svg>

    }
}

#[component]
pub fn EditButtonSVG(color: String) -> impl IntoView {
    view! {

        <svg
            class="recipe-menu-icon edit"
            fill=color
            viewBox="0 0 36.581608 38.501209"
            xml:space="preserve"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:svg="http://www.w3.org/2000/svg"
        >
            <g transform="translate(-46.505963,-219.95624)" >
                <path
                    d="M 68.210078,228.92536 50.504039,246.6314 c -2.333782,2.33378 -5.278889,9.35119 -3.401062,11.22902 1.877833,1.87783
                    8.895249,-1.06726 11.229033,-3.40105 l 17.706039,-17.70604 z m 8.370574,17.54561 c -0.308997,-0.78628 -1.417583,-0.78628
                    -1.72658,0 l -0.301237,0.76653 c -0.09433,0.24014 -0.283651,0.43019 -0.522824,0.52484 l -0.763784,0.30234 c -0.783383,0.31003
                    -0.783383,1.42287 0,1.73291 l 0.763784,0.30234 c 0.239173,0.0946 0.428491,0.28469 0.522824,0.52484 l 0.301237,0.76653 c
                    0.309002,0.78629 1.417579,0.78629 1.72658,0 l 0.301237,-0.76653 c 0.09433,-0.24015 0.283651,-0.4302 0.522824,-0.52484 l
                    0.763784,-0.30234 c 0.783383,-0.31004 0.783383,-1.42288 0,-1.73291 l -0.763784,-0.30234 c -0.239173,-0.0946 -0.428491,-0.2847
                    -0.522824,-0.52484 z M 50.515967,233.6425 c -0.308877,-0.78626 -1.417685,-0.78626 -1.726561,0 l -0.301198,0.76662 c -0.09445,0.24005
                    -0.283729,0.43008 -0.522921,0.52475 l -0.763649,0.30234 c -0.783422,0.31013 -0.783422,1.42297 0,1.73299 l 0.763649,0.30236
                    c 0.239192,0.0946 0.428473,0.28468 0.522921,0.52464 l 0.301198,0.76673 c 0.308876,0.7863 1.417684,0.7863 1.726561,0 l
                    0.301198,-0.76673 c 0.09445,-0.23996 0.283729,-0.43 0.522921,-0.52464 l 0.763841,-0.30236 c 0.78323,-0.31002 0.78323,-1.42286
                    0,-1.73299 l -0.763841,-0.30234 c -0.239192,-0.0947 -0.428473,-0.2847 -0.522921,-0.52475 z m 7.423789,-13.09656 c -0.309068,-0.78629
                    -1.417684,-0.78629 -1.726561,0 l -0.826231,2.10258 c -0.09445,0.24006 -0.283729,0.43009 -0.522921,0.52476 l -2.094755,0.82925
                    c -0.783422,0.31011 -0.783422,1.42286 0,1.73298 l 2.094755,0.82926 c 0.239192,0.0947 0.428473,0.28469 0.522921,0.52475 l
                    0.826039,2.10259 c 0.309069,0.78628 1.417685,0.78628 1.726753,10e-6 l 0.826039,-2.1026 c 0.09445,-0.24006 0.283729,-0.43007
                    0.522921,-0.52475 l 2.094756,-0.82926 c 0.783422,-0.31012 0.783422,-1.42287 0,-1.73298 l -2.094756,-0.82925 c -0.239192,-0.0947
                    -0.428473,-0.2847 -0.522921,-0.52476 z m 23.52659,2.9512 c 2.161637,2.16162 2.161637,5.66632 0,7.82794 l -3.047954,3.04795 c
                    -0.02289,-0.0265 -0.04687,-0.0523 -0.07201,-0.0774 l -7.678723,-7.67871 c -0.02507,-0.0251 -0.05081,-0.049 -0.07719,-0.0718
                    l 3.047934,-3.04792 c 2.161618,-2.16164 5.666322,-2.16164 7.82794,0 z"
                />
            </g>
        </svg>

    }
}

#[component]
pub fn DeleteButtonSVG(color: String) -> impl IntoView {
    view! {

        <svg
            class="recipe-menu-icon delete"
            fill=color
            viewBox="0 0 31.129204 37.321041"
            xml:space="preserve"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:svg="http://www.w3.org/2000/svg"
        >
            <g transform="translate(-172.45261,-221.2661)" >
                <g transform="matrix(1.9453981,0,0,1.9453981,164.8607,218.46035)" >
                    <path
                        d="m 19.90383,13.846162 c -0.02404,3.990033 -2.834482,6.073315 -4.6412,6.751 -0.4314,0.1342 -0.801615,-0.205445
                        -0.4569,-0.7396 1.008603,-1.562884 2.017118,-2.988466 1.725177,-5.660547 -0.146098,-1.337212 -1.21596,-3.273093
                        -3.413374,-4.5973222 -0.37488,-0.2259146 -0.758722,0.037186 -0.665124,0.4478122 0.493121,2.163376 -0.345054,4.027222
                        -0.754914,4.199134 -0.40986,0.171912 -1.303565,-0.673973 -1.611665,-1.054273 -0.1659999,-0.2048 -0.4659599,-0.2076
                        -0.6573899,-0.0264 -0.74636,0.7067 -1.679633,1.813352 -1.79128,3.1802 -0.1190041,1.456918 0.6367584,2.818413
                        1.2355681,3.482501 0.3254547,0.360933 0.3020165,1.001902 -0.60434,0.7251 C 6.0289214,19.762518 3.8405474,16.929113
                        3.9038301,13.846162 4,9.1610442 7.3726338,8.3124738 9.8598801,2.2230816 10.11953,1.5732916 10.623309,1.1460449
                        11.47653,1.6919916 c 3.748236,2.3983676 8.455149,7.5323539 8.4273,12.1541704 z"
                    />
                </g>
            </g>
        </svg>

    }
}

#[component]
pub fn LogoutButtonSVG() -> impl IntoView {
    view! {

        <svg
            class="logout-icon-svg settings-icon"
            viewBox="0 0 33.507629 37.23064"
            xml:space="preserve"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:svg="http://www.w3.org/2000/svg"
        >
            <g transform="translate(-129.87812,-222.17902)" >
                <g
                    transform="matrix(1.8615343,0,0,1.8615343,124.29352,218.45593)"
                >
                    <path
                        fill-rule="evenodd"
                        clip-rule="evenodd"
                        d="M 9.70725,2.4087 C 9,3.03569 9,4.18259 9,6.4764 v 11.0472 c 0,2.2938 0,3.4407 0.70725,4.0677 0.70725,0.627 1.78825,0.4384 3.95035,0.0613 l 2.3288,-0.4061 C 18.3809,20.8288 19.5781,20.62 20.2891,19.7417 21,18.8635 21,17.5933 21,15.0529 V 8.94711 C 21,6.40671 21,5.13652 20.2891,4.25826 19.5781,3.37999 18.3809,3.17118 15.9864,2.75354 L 13.6576,2.34736 C 11.4955,1.97026 10.4145,1.78171 9.70725,2.4087 Z M 12,10.1686 c 0.4142,0 0.75,0.3514 0.75,0.7849 v 2.093 c 0,0.4335 -0.3358,0.7849 -0.75,0.7849 -0.4142,0 -0.75,-0.3514 -0.75,-0.7849 v -2.093 c 0,-0.4335 0.3358,-0.7849 0.75,-0.7849 z"
                    />
                    <path
                        d="M 7.54717,4.5 C 5.48889,4.503 4.41599,4.54826 3.73223,5.23202 3,5.96425 3,7.14276 3,9.49979 v 5.00001 c 0,2.357 0,3.5355 0.73223,4.2678 0.68376,0.6837 1.75666,0.729 3.81494,0.732 C 7.49985,18.8763 7.49992,18.1557 7.50001,17.3768 V 6.6227 C 7.49992,5.84388 7.49985,5.1233 7.54717,4.5 Z"
                    />
                </g>
            </g>
        </svg>

    }
}

#[component]
pub fn BackupButtonSVG() -> impl IntoView {
    view! {

        <svg
            class="backup-icon-svg settings-icon"
            viewBox="0 0 37.087765 32.983551"
            xml:space="preserve"
            xmlns="http://www.w3.org/2000/svg"
        >
            <g transform="translate(-86.936566,-225.81565)" >
                <path
                    d="m 97.625879,225.81566 c 5.793791,0 7.271871,0.54452 8.641361,2.08633 0.87608,0.98631 1.19924,2.1545
                    1.83355,2.62034 0.63432,0.46584 1.95017,0.5017 7.07337,0.4651 6.47763,0 8.85017,1.92411 8.85017,6.64374
                    -0.0338,3.58586 -0.006,5.87879 -0.006,9.46485 0,2.06993 -0.10845,3.8599 -0.44259,5.36284 -0.33856,1.5241
                    -0.93336,2.86407 -1.98353,3.91424 -1.05017,1.05017 -2.39014,1.64496 -3.91422,1.98353 -1.50296,0.33411
                    -3.29293,0.44258 -5.36286,0.44258 H 98.639768 c -2.069995,0 -3.859943,-0.10845 -5.362872,-0.44258
                    -1.523995,-0.33857 -2.864076,-0.93336 -3.914249,-1.98353 -1.050154,-1.05017 -1.644882,-2.39014 -1.983574,-3.91424
                    -0.334,-1.50294 -0.442503,-3.29291 -0.442503,-5.36284 v -9.96776 c 0,-2.06999 0.108505,-3.85993 0.442503,-5.36287
                    0.338692,-1.52398 0.93342,-2.86407 1.983574,-3.91424 1.050173,-1.05017 2.390069,-2.03549 8.263232,-2.03549 z m
                    9.706001,14.44237 c 0,-1.02404 -0.83009,-1.85413 -1.85412,-1.85413 -1.02404,0 -1.85413,0.83009 -1.85413,1.85413
                    v 5.72143 l -1.47012,-1.47013 c -0.72407,-0.72403 -1.89803,-0.72403 -2.622115,0 -0.724071,0.72403 -0.724071,1.89806
                    0,2.62209 l 4.519435,4.51942 c 0.0198,0.0198 0.0401,0.0393 0.0607,0.0582 0.33893,0.36916 0.82545,0.60074 1.36629,0.60074
                    0.54085,0 1.02737,-0.23158 1.3663,-0.60074 0.0206,-0.0189 0.0409,-0.0384 0.0607,-0.0582 l 4.51943,-4.51942 c
                    0.72403,-0.72403 0.72403,-1.89806 0,-2.62209 -0.72403,-0.72403 -1.89807,-0.72403 -2.6221,0 l -1.47012,1.47013 z"
                />
            </g>
        </svg>

    }
}

/// Close Tags Menu Button
#[component]
pub fn CrossButtonSVG(#[prop(optional)] add_class: String) -> impl IntoView {
    view! {
        <svg
            class=add_class
            viewBox="0 -0.5 25 25"
        >
            <path
                d="M6.96967 16.4697C6.67678 16.7626 6.67678 17.2374 6.96967 17.5303C7.26256 17.8232 7.73744
                17.8232 8.03033 17.5303L6.96967 16.4697ZM13.0303 12.5303C13.3232 12.2374 13.3232 11.7626
                13.0303 11.4697C12.7374 11.1768 12.2626 11.1768 11.9697 11.4697L13.0303 12.5303ZM11.9697
                11.4697C11.6768 11.7626 11.6768 12.2374 11.9697 12.5303C12.2626 12.8232 12.7374 12.8232
                13.0303 12.5303L11.9697 11.4697ZM18.0303 7.53033C18.3232 7.23744 18.3232 6.76256 18.0303
                6.46967C17.7374 6.17678 17.2626 6.17678 16.9697 6.46967L18.0303 7.53033ZM13.0303 11.4697C12.7374
                11.1768 12.2626 11.1768 11.9697 11.4697C11.6768 11.7626 11.6768 12.2374 11.9697 12.5303L13.0303
                11.4697ZM16.9697 17.5303C17.2626 17.8232 17.7374 17.8232 18.0303 17.5303C18.3232 17.2374 18.3232
                16.7626 18.0303 16.4697L16.9697 17.5303ZM11.9697 12.5303C12.2626 12.8232 12.7374 12.8232 13.0303
                12.5303C13.3232 12.2374 13.3232 11.7626 13.0303 11.4697L11.9697 12.5303ZM8.03033 6.46967C7.73744
                6.17678 7.26256 6.17678 6.96967 6.46967C6.67678 6.76256 6.67678 7.23744 6.96967 7.53033L8.03033
                6.46967ZM8.03033 17.5303L13.0303 12.5303L11.9697 11.4697L6.96967 16.4697L8.03033 17.5303ZM13.0303
                12.5303L18.0303 7.53033L16.9697 6.46967L11.9697 11.4697L13.0303 12.5303ZM11.9697 12.5303L16.9697
                17.5303L18.0303 16.4697L13.0303 11.4697L11.9697 12.5303ZM13.0303 11.4697L8.03033 6.46967L6.96967
                7.53033L11.9697 12.5303L13.0303 11.4697Z"
            />
        </svg>
    }
}

#[component]
pub fn PlusIconSVG(#[prop(optional)] add_class: String) -> impl IntoView {
    view! {
        <svg
            class="plus-icon-svg ".to_string() + &add_class
            viewBox="0 0 24.00 24.00"
        >
            <path
                d="M19,11H13V5a1,1,0,0,0-2,0v6H5a1,1,0,0,0,0,2h6v6a1,1,0,0,0,2,0V13h6a1,1,0,0,0,0-2Z"
            />
        </svg>
    }
}

#[component]
pub fn HashtagSVG(#[prop(optional)] add_class: String) -> impl IntoView {
    view! {
        <svg
            class=add_class
            viewBox="0 0 4.07266 4.3041706"
            xml:space="preserve"
            xmlns="http://www.w3.org/2000/svg"
            xmlns:svg="http://www.w3.org/2000/svg"
        >
            <g transform="translate(-134.75325,-48.94557)" >
                <path
                    style="stroke-linecap:round;stroke-linejoin:round;"
                    d="m 136.37617,49.245571 -0.69453,3.70417 m 2.25722,-3.70417 -0.66145,3.70417 m 1.00045,-1.174091 h -3.22461 m 3.47266,-1.422135 h -3.25769"
                />
            </g>
        </svg>
    }
}

#[component]
pub fn SaveIconSVG() -> impl IntoView {
    view! {
        <img src="/assets/save.svg" class="save-img" />
    }
}
