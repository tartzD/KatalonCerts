<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_css variables root     --wmui-base100 f_1255e8</name>
   <tag></tag>
   <elementGuidId>e9b8acb7-b202-4ec7-bc4d-1d7a39c4346b</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.vector-sitenotice-container</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[2]/div/div</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>e2cd491f-b42a-4423-a4d0-71f96dd28a54</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>vector-sitenotice-container</value>
      <webElementGuid>da1e0182-e953-4f1e-8df8-01516a12e154</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
			

/* css variables */
:root {
    --wmui-base100: #fff;
    --wmui-base90: #f8f9fa;
    --wmui-base80: #eaecf0;
    --wmui-base70: #c8ccd1;
    --wmui-base50: #a2a9b1;
    --wmui-base30: #72777d;
    --wmui-base20: #54595d;
    --wmui-base10: #202122;
    --wmui-base0: #000;
    --wmui-accent: #36c;
    --wmui-accent-light: #eaf3ff;
    --wmui-accent-dark: #2a4b8d;
    --wmui-red: #d33;
    --wmui-red-light: #fee7e6;
    --wmui-red-dark: #b32424;
    --wmui-green: #00af89;
    --wmui-green-light: #d5fdf4;
    --wmui-green-dark: #14866d;
    --wmui-yellow: #fc3;
    --wmui-yellow-light: #fef6e7;
    --wmui-yellow-dark: #ac6600;
    --frb-primary: #2e5cb8;
    --frb-primary-light: #dde4f3;
    --frb-primary-dark: #2a4b8d;
    --frb-body: var(--wmui-base0);
    --frb-link: var(--wmui-accent);
    --frb-link-hover: #447ff5;
    --frb-message-background: var(--wmui-base100);
    --frb-message: var(--wmui-base0);
    --frb-message-border: #900;
    --frb-muted: var(--wmui-base20);
    --frb-muted-hover: var(--wmui-base0);
    --frb-radio: var(--wmui-accent);
    --frb-button: var(--wmui-base90);
    --frb-button-border: var(--wmui-base50);
    --frb-button-hover: var(--wmui-accent-light);
    --frb-button-border-hover: var(--wmui-base50);
    --frb-button-focus: var(--wmui-accent-light);
    --frb-button-border-focus: var(--wmui-base50);
    --frb-button-selected: var(--frb-primary-dark);
    --frb-button-border-selected: var(--frb-primary-dark);
    --frb-submit: var(--wmui-accent);
    --frb-submit-border: var(--wmui-accent);
    --frb-submit-hover: #447ff5;
    --frb-focus: var(--wmui-accent);
    --frb-error: var(--wmui-red);
    --frb-katie-gold: #ffcc00;
}

/* Hide when editing */
.action-edit #centralNotice,
.ve-activated #centralNotice {
    display: none !important;
}

/* Fix fixed position z-index for de.wikipedia and 'gesproken_wiki' element on nl.wikipedia */
.mw-body { z-index: auto; }
#siteNotice { z-index: 3; }

/* Border-Box */

.frb,
.frb *,
.frb *:before,
.frb *:after {
    -moz-box-sizing: border-box;
    -webkit-box-sizing: border-box;
    box-sizing: border-box;
}

/* Banner wide settings */

.frb input,
.frb button {
    font-size: inherit;
    font-family: inherit;
}

.frb button {
    cursor: pointer;
    border: 0;
    background: transparent;
    padding: 0;
}

.frb frb-amt,
.frb-replaced {
    white-space: nowrap;
}

@media (prefers-reduced-motion: reduce) {
    .frb,
    .frb * {
        transition-duration: 0.01ms !important;
    }
}

/* --- Main banner wrapper --- */

.frb {
    display: none;
    background-color: var(--wmui-base100);
    color: var(--frb-body);
    font-size: 16px;
    line-height: 1.1875; /*19px @16px*/
    font-family: system-ui, -apple-system,BlinkMacSystemFont, &quot;Segoe UI&quot;, Roboto, &quot;Oxygen-Sans&quot;, Ubuntu, Cantarell, Lato, &quot;Helvetica Neue&quot;, Helvetica, Arial, sans-serif;
    text-align: left; /* needed because of #siteNotice { text-align: center; } in MediaWiki */
    font-weight: normal;
    font-style: normal; /* needed for uk.wikipedia */
}

body.rtl .frb {
    text-align: right;
}

.frb-in-article {
    margin-bottom: 20px;
}

.frb-nag,
.frb-fixed {
    position: fixed;
    bottom: 0;
    left: 0;
    width: 100%;
    padding: 16px;
    background-color: var(--wmui-base100);
    border-top: 1px solid var(--wmui-base70);
    box-shadow: 0 -1px 1px rgba(0,0,0,0.1);
}

/* Avoid overlapping Vector 2022's fixed width toggle */
@media ( min-width: 1400px ) {
    body.skin-vector-2022 .frb-nag,
    body.skin-vector-2022 .frb-fixed {
        left: 48px;
        right: 48px;
        width: auto;
        border-left: 1px solid var(--wmui-base70);
        border-right: 1px solid var(--wmui-base70);
    }
}

.frb-layout {
    display: grid;
    grid-template-columns: auto 330px;
    grid-template-rows: 420px auto;
    grid-template-areas:
        &quot;main sidebar&quot;
        &quot;footer sidebar&quot;;
}

@media (max-width: 959px) {
    .frb-layout {
        grid-template-rows: auto auto;
    }
}

/* --- Icon buttons --- */

.frb .frb-icon-btn {
    display: block;
    cursor: pointer;
    background-repeat: no-repeat;
    background-position: center;
    opacity: .55;
}
.frb .frb-icon-btn:hover {
    opacity: 1;
}

.frb .frb-close {
    position: absolute;
    top: 0;
    right: 0;
    width: 45px;
    height: 45px;
    background-image: url(&quot;data:image/svg+xml,%3Csvg viewBox='0 0 10 10' xmlns='http://www.w3.org/2000/svg'%3E%3Cg stroke='%23000' fill='none' stroke-linecap='round'%3E%3Cpath d='M1 1 l8 8 M9 1 l-8 8'%3E%3C/path%3E%3C/g%3E%3C/svg%3E&quot;);
    background-size: 16px 16px;
}
.frb .frb-step.frb-step-1 .frb-close {
    top: -15px;
    right: -15px;
}
body.rtl .frb .frb-close {
    right: auto;
    left: 0;
}
.frb-nag-layout .frb-close {
    top: -17px;
    right: -17px;
}

.frb .frb-back {
    position: absolute;
    top: 0;
    left: 0;
    width: 45px;
    height: 45px;
    background-image: url(&quot;data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 16'%3E%3Cg fill='none' fill-rule='evenodd' transform='translate(1 1)'%3E%3Cpath stroke='%23000' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.778' d='M7.181 13.285L.753 7 7.181.715'%3E%3C/path%3E%3Crect fill='%23000' width='18.182' height='1.778' x='.818' y='6.111' rx='.889'%3E%3C/rect%3E%3C/g%3E%3C/svg%3E&quot;);
    background-size: 20px 13px;
}
body.rtl .frb .frb-back {
    left: auto;
    right: 0;
    transform: rotate(180deg);
}

/* --- RML Back button --- */

.back-rml, .frb-nag .frb-rml-close-wrapper {
    display: none;
}
.frb-nag .back-rml {
    text-align: center;
    margin: 0 auto;
    padding: 2px;
    font-size: 11px;
    line-height: 1;
    text-transform: uppercase;
    cursor: pointer;
    color: var(--frb-muted);
    height: 45px;
}
.frb-nag .back-rml:hover {
    color: var(--frb-muted-hover);
}

.frb-rml-displayed .back-rml {
    display: block;
}

/* --- RML Close button --- */

.frb-rml-close-wrapper {
    text-align: center;
}

.frb-rml-close-wrapper button {
    font-size: 12px;
    color: var(--frb-muted);
    height: 45px;
}

.frb-rml-close-wrapper:hover button {
    color: var(--frb-muted-hover);
}

.frb-rml-close-icon {
    width: 10px;
    height: 10px;
    margin-bottom: -1px;
}

.frb-rml-close-icon g {
    stroke: currentColor;
}

/* -------------- Message -------------- */

.frb-message {
    grid-area: main;
    display: flex;
    flex-direction: column;
    justify-content: center;
    position: relative;
    border-radius: .75em;
    background: #308557;
    border: 6px solid #308557;
    color: var(--wmui-base100);
    font-weight: normal;
    font-size: 14px;
    line-height: 1.6; /*24px @15px*/
    z-index: 1;
    padding: 20px;
}

@media (min-width: 860px) {
    .frb-message {
        padding: 10px 20px 20px;
    }
}

@media (min-width: 960px) {
    .frb-message {
        font-size: 1.175vw;
    }
}

@media (min-width: 1200px) {
    .frb-message {
        font-size: 1.1vw;
    }
}

@media (min-width: 1400px) {
    .frb-message {
        font-size: 1vw;
    }
}

@media (min-width: 1800px) {
    .frb-message {
        font-size: 18px;
    }
}

.frb-message-icon {
    float: left;
    margin-top: 4px; /*in px since margin is consistent on all bp*/
    margin-right: 2px;
    height: 1em;
    width: 1em;
}

.frb-message-icon circle {
    fill: #FEFD34;
}

.frb-nag .frb-message {
    border: 6px solid var(--frb-message-border);
}

.frb-nag .frb-message-icon circle {
    fill: var(--frb-message-border);
}
.frb-message-icon path {
    fill: var(--wmui-base0);
}

.frb-nag .frb-message-icon path {
    fill: var(--wmui-base100);
}

.frb-nag .frb-message-icon {
    margin-top: 3px;
}

@media all and (min-width: 1300px) {
    .frb-nag .frb-message-icon {
        margin-top: 4px;
    }
}

body.rtl .frb-message-icon {
    float: right;
    margin-right: 0;
    margin-left: 4px;
}

.frb-greeting .frb-message-icon {
    float: none;
    margin-right: 0;
    margin-top: 0;
    margin-bottom: -2px;
}

.frb-message p {
    margin: 0 0 1em;
}

.frb-message p:last-child {
    margin: 0;
}

.frb-greeting {
    flex: 0 0 auto;
    max-height: 62px;
    margin-bottom: 0.5em;
    text-align: center;
    font-size: 1.75em;
    line-height: 1;
    font-weight: bold;
}

.frb-subgreeting {
    font-size: 0.6em;
    line-height: 1.6;
    font-weight: normal;
}

.frb-message-content {
    padding-bottom: 25px;
}

@media (max-width: 959px) {
    .frb-greeting {
        max-height: none;
    }
    .frb-message-content {
        font-size: 14px !important;
    }
}

@media (max-width: 860px) {
    .frb-message-content {
        padding-bottom: 55px;
    }
}


/* Nag styles */

.frb-nag {
    cursor: pointer;
}

.frb-nag-layout {
    display: grid;
    grid-template-areas: &quot;main sidebar&quot;;
    grid-template-columns: auto 360px;
}

@media (min-width: 1200px) {
    .frb-nag-layout {
        grid-template-columns: auto 440px;
    }
}

.frb-nag .frb-message {
    padding: 22px 26px;
    background: var(--frb-message-background);
    color: var(--frb-message);
}

@media (max-width: 1400px) {
    .frb-nag .frb-message {
        padding: 18px 18px;
        font-size: 16px;
    }
}

@media (max-width: 1100px) {
    .frb-nag .frb-message {
        font-size: 14px;
    }
}

.frb-nag .frb-form-wrapper {
    padding: 0 16px;
    display: flex;
    align-items: center;
    justify-content: center;
}

/* Triangle */
.frb-message::after {
    position: absolute;
    content: &quot; &quot;;
    top: 180px;
    right: -25px;
    margin: -10px 0 0 0;
    border: 10px solid transparent;
    border-left-color: #308557;
    pointer-events: none;
}
body.rtl .frb-message::after {
    right: auto;
    left: -25px;
    border-left-color: transparent;
    border-right-color: #308557;
}

.frb-nag .frb-message::after {
    top: 50%;
    border-left-color: var(--frb-message-border);
}

body.rtl .frb-nag .frb-message::after {
    border-left-color: transparent;
    border-right-color: var(--frb-message-border);
}

/* -------------- Form -------------- */

.frb-form-wrapper {
    grid-area: sidebar;
    position: relative;
    background: var(--wmui-base100);
}

.frb-form-wrapper fieldset {
    border: 0;
    margin: 0 auto;
    padding: 0 0 6px 0;
}

.frb-form-wrapper .frb-amounts {
    padding: 0;
    margin-top: 8px;
}

.frb-form-wrapper legend,
.frb-rml-form-legend {
    display: block;
    margin: 0 0 2px;
    padding: 0 4px;
    font-weight: normal;
    text-align: inherit;
    font-size: 14px;
    line-height: 1.2142857143; /*17px @14px*/
    color: var(--frb-muted);
    transition: all .25s ease-in-out;
}

.frb-form-wrapper {
    counter-reset: count;
}
.frb-numbered {
    counter-increment: count;
}
.frb-numbered::before {
    content: counter(count) &quot;. &quot;;
    position: absolute;
    left: -12px;
}
body.rtl .frb-numbered::before {
    left: auto;
    right: -12px;
}

.frb-rml-form-legend {
    padding: 0 0 2px;
}

.frb-frequency legend,
.frb-amounts legend {
    padding: 0 5px;
}

.frb-form-wrapper fieldset:first-of-type legend {
    padding-top: 0;
}

.frb-form-wrapper ul {
    display: flex;
    flex-wrap: wrap;
    width: 100%;
    margin: 0;
    padding: 0;
    list-style: none;
}

.frb-form-wrapper li {
    display: inline-flex;
    justify-content: flex-start;
    align-items: center;
    margin: 0;
}

#frb-form {
    padding-left: 25px;
    position: relative;
    overflow: hidden;
    height: 100%; /* ensure varying height steps don't get cut off */
}
body.rtl #frb-form {
    padding-left: 0;
    padding-right: 25px;
}

.frb-frequency ul li {
    flex: 1 0 0;
}

.frb-amounts ul li {
    flex: 0 0 32%;
    max-width: 32%;
}

.frb-amounts ul li.frb-amt-other {
    flex: 0 0 67%;
    max-width: 67%;
}

.frb-amounts .frb-radio-label {
    white-space: nowrap;
}

/* --- Common Button Styles --- */

/* Hide radio buttons */
.frb-form-wrapper .frb-methods input[type=&quot;radio&quot;],
.frb-form-wrapper .frb-optin input[type=&quot;radio&quot;],
.frb-form-wrapper input[type=&quot;checkbox&quot;] {
    position: absolute;
    overflow: hidden;
    height: 1px;
    width: 1px;
    clip: rect(0 0 0 0);
    border: 0;
    margin: -1px;
    padding: 0;
}

/* TODO: are these frb-btn styles needed? */
.frb-btn {
    width: 100%;
    height: 48px;
    display: block;
    background-color: var(--frb-button);
    color: var(--frb-body);
    font-size: 16px;
    line-height: 1.25; /*20px @16px*/
    padding: 13px 4px 15px 4px;
    border: 1px solid var(--frb-button-border);
    border-radius: 2px;
    outline: 0;
    text-align: center;
    cursor: pointer;
    font-family: inherit;
    font-weight: 500;
    transition: all .25s ease-in-out;
}
.frb-btn:hover {
    background-color: var(--wmui-base100);
    color: #444;
    border-color: var(--wmui-base50);
}
.frb-btn:active {
    background-color: #d9d9d9;
    color: var(--wmui-base0);
    border-color: #7d8389;
}
.frb-form-wrapper input[type=radio]:checked + .frb-btn {
    background-color: var(--frb-primary-dark);
    color: var(--wmui-base100);
    border-color: #7d8389;
}

.frb-rml-link {
    display: block;
    font-size: 16px;
    line-height: 1.125; /*18px @16px*/
    color: var(--frb-link);
    margin: 16px auto 0;
    text-align: center;
    font-weight: bold;
}

.frb-rml-link:hover,
.frb-rml-link:focus {
    color: var(--frb-link-hover);
}

.frb-radio,
.frb-radio-label {
    font-size: 16px;
    line-height: 1.375; /*22px @16px*/
}

.frb-radio {
    cursor: pointer;
    margin: 0 0 0 7px;
}

.frb-radio-label {
    display: block;
    padding: 12px 7px;
    cursor: pointer;
    font-weight: bold;
    flex: 1 0 auto;
}

/* Focus styles */

/*Outline reset*/
.frb-form-wrapper input[type=radio]:focus,
.frb-radio:focus + .frb-radio-label,
#frb-amt-other-input:focus,
#frb-rml-email:focus {
    outline: 0;
}

.frb button:focus,
.frb-btn:focus,
.frb-icon-btn:focus,
.frb-btn-submit:focus,
.frb-form-wrapper input[type=radio]:focus + .frb-btn,
.frb-form-wrapper input[type=radio]:focus + #frb-amt-other-label,
#frb-amt-other-input:focus,
.frb-rml-displayed .frb-rml-form input:focus,
.frb-nag-btn:focus,
#nag-rml-btn:focus {
    outline: 0;
    border-color: var(--frb-focus) !important;
    box-shadow: inset 0 0 0 2px var(--frb-focus);
}

.frb button.frb-submit:focus {
    box-shadow: inset 0 0 0 1px var(--frb-submit-hover), inset 0 0 0 2px var(--wmui-base100);
}

.frb-rml-displayed .frb-rml-form input:focus,
.frb-rml-displayed .frb-rml-form .frb-btn-submit:focus {
    position: relative;
}

.frb-radio:enabled:focus + .frb-radio-label,
.frb-radio:enabled:hover + .frb-radio-label {
    color: var(--frb-link);
    text-decoration: underline;
}

.frb-radio:disabled + label {
    opacity: 0.4;
    cursor: default;
}

#frb-amt-other-input:focus,
#frb-amt-other-input:hover {
    box-shadow: none;
    box-shadow: 0 2px #fff, 0 3px var(--frb-link);
    color: var(--frb-link);
}

.frb-form-wrapper input[type=radio]:focus + .frb-btn,
.frb-form-wrapper input[type=radio]:focus + #frb-amt-other-label,
.frb-form-wrapper input[type=radio]:checked + .frb-btn:focus,
.frb-form-wrapper input[type=radio]:checked + #frb-amt-other-label:focus,
.frb-form-wrapper .frb-btn-submit:focus,
#nag-yes-btn:focus {
    box-shadow: inset 0 0 0 1px var(--frb-submit), inset 0 0 0 2px var(--wmui-base100);
}

.frb-btn img {
    padding: 0 4px;
    max-width: 100%;
}

.frb-methods .frb-btn {
    height: 64px;
    line-height: 1.125; /*18px @16px*/
}

.frb-methods svg {
    max-width: 100%;
    width: 64px;
}

/*Slight adaption for Paypal logo with USD string*/
.frb-methods .frb-logo-payments--paypal-usd {
    width: 85px;
    margin-bottom: -6px;
}

/* -- Credit card logos -- */

.frb-cc-logo-wrapper {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    margin: 0 auto;
    max-width: 80px;
    font-size: 0; /* Remove spacing between icons */
}

.frb-pm-cc svg {
    flex: 0 0 24px;
    max-width: 24px;
    width: 24px;
    max-height: 15px; /* height needed for IE11 */
    margin: 2px;
    display: none;
}

/* Reduce card logo spacing/sizing when there are 4 methods */
.frb-payment-options .frb-button:first-child:nth-last-child(4) .frb-cc-logo-wrapper,
.frb-payment-options .frb-button:first-child:nth-last-child(4) ~ .frb-button .frb-cc-logo-wrapper {
    width: 61px;
}

.frb-payment-options .frb-button:first-child:nth-last-child(4) .frb-cc-logo-wrapper svg,
.frb-payment-options .frb-button:first-child:nth-last-child(4) ~ .frb-button .frb-cc-logo-wrapper svg {
    width: 24px;
    height: 15px;
}

/* Countries with 3 card types */
.frb-cctypes-vma .frb-cc-logo-wrapper {
    width: 100%;
    flex-wrap: nowrap;
}
.frb-cctypes-vma svg  {
    flex: 0 0 28%;
    max-width: 28%;
    width: 28%;
    max-height: 34px;
}

.frb-cc-logo-wrapper {
    display: none;
}

.frb-cctypes-vmad .frb-cc-logo-wrapper,
.frb-cctypes-vmaj .frb-cc-logo-wrapper,
.frb-cctypes-vma  .frb-cc-logo-wrapper,
.frb-cctypes-vm   .frb-cc-logo-wrapper {
    display: flex;
}

.frb-cctypes-vmad .frb-pm-cc-label,
.frb-cctypes-vmaj .frb-pm-cc-label,
.frb-cctypes-vma  .frb-pm-cc-label,
.frb-cctypes-vm   .frb-pm-cc-label {
    display: none;
}

.frb-cctypes-vmad .frb-cc-logo-visa,
.frb-cctypes-vmad .frb-cc-logo-mastercard,
.frb-cctypes-vmad .frb-cc-logo-amex,
.frb-cctypes-vmad .frb-cc-logo-discover,

.frb-cctypes-vmaj .frb-cc-logo-visa,
.frb-cctypes-vmaj .frb-cc-logo-mastercard,
.frb-cctypes-vmaj .frb-cc-logo-amex,
.frb-cctypes-vmaj .frb-cc-logo-jcb,

.frb-cctypes-vma  .frb-cc-logo-visa,
.frb-cctypes-vma  .frb-cc-logo-mastercard,
.frb-cctypes-vma  .frb-cc-logo-amex,

.frb-cctypes-vm   .frb-cc-logo-visa,
.frb-cctypes-vm   .frb-cc-logo-mastercard {
    display: inline-block;
}

/* Submit/Continue buttons (blue background) */
.frb .frb-btn-submit {
    width: 100%;
    display: block;
    margin-top: 6px;
    padding: 8px;
    color: var(--wmui-base100);
    background-color: var(--frb-submit);
    border-color: var(--frb-submit);
    cursor: pointer;
    border: 0;
    border-radius: 2px;
    font-size: 16px;
    font-weight: bold;
    transition: all .25s ease-in-out;
    height: 45px;
}
.frb .frb-btn-submit:hover {
    background-color: var(--frb-submit-hover);
    border-color: var(--frb-submit-hover);
}
.frb .frb-btn-submit:active {
    background-color: var(--frb-primary-dark);
    border-color: var(--frb-primary-dark);
    box-shadow: none;
}

.frb-submit-txt-once { display: inline; }
.form-monthly .frb-submit-txt-once { display: none; }

.frb-submit-txt-monthly { display: none; }
.form-monthly .frb-submit-txt-monthly { display: inline; }

/* --- Other Amount --- */

/* TODO: Can we cut these down at all? */
#frb-amt-other-input::-webkit-input-placeholder {
    color: var(--frb-body);
}
#frb-amt-other-input::-moz-placeholder {
    opacity: 1;
    color: var(--frb-body);
}
#frb-amt-other-input:-ms-input-placeholder {
    color: var(--frb-body);
}
#frb-amt-other-input:-moz-placeholder {
    opacity: 1;
    color: var(--frb-body);
}

#frb-amt-other-input:focus::-webkit-input-placeholder {
    color: #666;
}
#frb-amt-other-input:focus::-moz-placeholder {
    opacity: 1;
    color: #666;
}
#frb-amt-other-input:focus:-ms-input-placeholder {
    color: #666;
}
#frb-amt-other-input:focus:-moz-placeholder {
    opacity: 1;
    color: #666;
}

#frb-amt-other-input:hover::-webkit-input-placeholder {
    color: var(--frb-link);
}
#frb-amt-other-input:hover::-moz-placeholder {
    opacity: 1;
    color: var(--frb-link);
}
#frb-amt-other-input:hover:-ms-input-placeholder {
    color: var(--frb-link);
}
#frb-amt-other-input:hover:-moz-placeholder {
    opacity: 1;
    color: var(--frb-link);
}

.frb-amt-other {
    width: 66.666666%;
}

#frb-amt-other-input {
    width: calc(100% - 33px);
    padding: 4px 2px 4px 7px;
    border: none;
    direction: ltr;
    text-align: left;
    font-size: 16px;
    font-family: inherit;
    font-weight: bold;
    color: var(--frb-body);
    box-shadow: 0 2px #fff, 0 3px var(--frb-body);
    -webkit-appearance: none;
    border-radius: 0; /* needed for iPad */
}

/* TODO: What is this for? */
@media all and (min-width: 1280px) {
    .frb-amt-other #frb-amt-other-input {
        margin: 0;
    }
}

body.rtl #frb-amt-other-input {
    text-align: right;
}

/* --- Transaction fees options --- */

/* Checkbox styles */

.frb-checkbox-label {
    position: relative;
    display: inline-block;
    margin: 13px 0 0 5px;
    width: calc(100% - 10px);
    padding-left: 26px;
    padding-top: 2px;
    cursor: pointer;
    font-size: 14px;
    line-height: 1.3571428571; /*19px @14px*/
    color: var(--frb-body);
}
body.rtl .frb-checkbox-label {
    margin: 13px 5px 0 0;
    padding-left: 0;
    padding-right: 26px;
}

/* Outer-box */
.frb-checkbox-label::before {
    position: absolute;
    content: &quot;&quot;;
    top: 3px;
    left: 0;
    display: inline-block;
    height: 17px;
    width: 17px;
    border-radius: 2px;
    border: 1px solid var(--wmui-base50);
    background-color: var(--wmui-base90);
}
body.rtl .frb-checkbox-label::before {
    left: auto;
    right: 0;
}

/* Checkmark */
.frb-checkbox-label::after {
    position: absolute;
    content: &quot;&quot;;
    top: 8px;
    left: 4px;
    display: inline-block;
    height: 5px;
    width: 9px;
    border-left: 2px solid;
    border-bottom: 2px solid;
    transform: rotate(-45deg);
    border-color: var(--wmui-base100);
}
body.rtl .frb-checkbox-label::after {
    left: auto;
    right: 4px;
}

.frb-ptf-total {
    font-weight: bold;
}
.frb-ptf-fee {
    white-space: nowrap;
}

/* Hide the checkmark by default */
.frb-checkbox + .frb-checkbox-label::after {
    content: none;
}
/* Unhide the checkmark on the checked state */
.frb-checkbox:checked + .frb-checkbox-label::after {
    content: &quot;&quot;;
}

.frb-checkbox:checked + .frb-checkbox-label:before {
    background-color: var(--wmui-accent);
    border-color: var(--wmui-accent);
}

/* Focus styles - unchecked */
.frb-checkbox:focus + .frb-checkbox-label::before {
    border-color: var(--wmui-accent);
    box-shadow: inset 0 0 0 1px var(--wmui-accent);
}

/* Focus styles - checked */
.frb-checkbox:focus:checked + .frb-checkbox-label::before {
    box-shadow: inset 0 0 0 1px var(--wmui-accent), inset 0 0 0 2px var(--wmui-base100);
}

/* Hover */
.frb-checkbox:hover + .frb-checkbox-label::before {
    background-color: var(--wmui-base80);
}

.frb-checkbox:checked:hover + .frb-checkbox-label:before {
    background-color: var(--frb-link-hover);
    border-color: var(--frb-link-hover);
}

/* --- Email opt-in --- */

.frb-form-wrapper .frb-optin {
    margin-bottom: 3px;
}

.frb-form-wrapper .frb-optin legend {
    margin-bottom: 8px;
    display: inline-block;
}

.frb-optin .frb-radio {
    margin: 6px 5px 0px 9px;
}

.frb-optin .frb-radio-label {
    float: right;
    width: calc(100% - 30px);
    white-space: normal;
    font-size: 14px;
    line-height: 1.3571428571; /*19px @14px*/
    font-weight: bold;
}

.frb-optin .frb-radio-label:hover,
.frb-optin .frb-radio:hover + .frb-radio-label {
    text-decoration: none;
}

.frb-optin-no-prompt {
    display: none;
    margin: 8px;
    padding: 4px 8px;
    border: 2px solid #900;
    border-radius: 2px;
    font-size: 14px;
    line-height: 1.2857142857; /*18px @14px*/
    font-weight: normal;
}

.frb-optin-no-prompt.is-positive {
    border-color: var(--wmui-green-dark);
    font-weight: bold;
}

.frb-optin-no-prompt__yes {
    display: none;
}

.frb-optin-no-prompt__no {
    display: block;
}

.frb-optin-no-prompt.is-positive .frb-optin-no-prompt__yes {
    display: block;
}

.frb-optin-no-prompt.is-positive .frb-optin-no-prompt__no {
    display: none;
}

.frb-optin-smallprint {
    padding: 8px 8px 0 8px;
}

/* --- Payment method Buttons --- */

/* Fade methods which aren't monthly capable when monthly option is selected */
.form-monthly .no-monthly {
    opacity: 0.4 !important;
}

.form-monthly .no-monthly .frb-label {
    cursor: default;
}

.frb-form-wrapper .frb-methods {
    margin-top: 12px;
    padding-bottom: 0;
}

/* --- Footer / Small Print --- */
.frb-smallprint {
    font-size: 12px;
    line-height: 1.5; /*18px @12px*/
    color: var(--frb-muted);
    font-weight: normal;
}
.frb-smallprint a {
    color: var(--frb-muted);
    text-decoration: underline;
}
.frb-smallprint a:hover {
    color: var(--frb-muted-hover);
}

.frb-footer {
    grid-area: footer;
    padding-top: 8px;
    display: flex;
}

.frb-footer-message {
    flex: 1 1 100%;
    max-width: calc(100% - 160px);
    display: inline-block;
    padding-right: 45px;
}

.frb-footer-button {
    display: inline-flex;
    align-items: flex-start;
    flex: 0 0 160px;
    max-width: 160px;
    width: 160px;
    justify-content: flex-end;
    margin-top: 20px;
}

.frb-footer-button .frb-already-donated-button {
    font-size: 14px;
    font-weight: bold;
    display: inline-block;
    text-decoration: none;
    color: var(--frb-muted);
}
.frb-footer-button .frb-already-donated-button:hover {
    color: var(--wmui-base0);
}

@media (max-width: 1160px) {
    .frb-smallprint {
        font-size: 10px;
    }
}

/* --- Disable I already donated --- */
.frb.frb-iad-disabled .frb-iad {
    display: none;
}

/* --- Show and Hiding (Minimize and Maximize) --- */

.frb-nag-btns {
    display: flex;
    flex: 1 0 0;
}

.frb.frb-rml-displayed .frb-nag-btns {
    display: none;
}

button.frb-nag-btn {
    flex: 1 0 0;
    margin: 0 8px;
    padding: 6px;
    min-height: 48px;
    background-color: white;
    border: 1px solid var(--frb-button-border);
    border-radius: 2px;
    outline: 0;
    color: var(--frb-body);
    font-weight: normal;
    font-size: 13px;
    line-height: 1.3;
    transition: all 100ms;
    cursor: pointer;
}

@media all and (min-width: 1100px) {
    button.frb-nag-btn {
        font-size: 14px;
    }
}

@media all and (min-width: 1200px) {
    button.frb-nag-btn {
        font-size: 16px;
    }
}

button.frb-nag-btn:hover {
    transform: scale(1.043478);
}

#nag-yes-btn {
    font-weight: bold;
    background-color: var(--frb-submit);
    border: 1px solid var(--frb-submit);
    color: white;
}

/* --- Maybe later --- */

.frb-rml-disabled .frb-rml,
.frb-rml-disabled .frb-rml-link,
.frb-rml-disabled #nag-rml-btn {
    display: none;
}

#frb-main .frb-rml {
    position: absolute;
    width: 100%;
}

.frb-rml-form-wrapper {
    display: none;
    position: absolute;
    margin-top: 12px;
    margin-left: 12px;
    width: calc(100% - 12px);
    padding: 16px 16px 0 16px;
    text-align: left;
    background: var(--wmui-base100);
    border: 1px solid var(--wmui-base70);
    border-radius: 2px;
    box-shadow: 0 2px 2px 0 rgba(0, 0, 0, 0.15);
    z-index: 10;
}

.frb-rml-form-wrapper:after,
.frb-rml-form-wrapper:before {
    bottom: 100%;
    left: 50%;
    border: solid transparent;
    content: &quot; &quot;;
    height: 0;
    width: 0;
    position: absolute;
    pointer-events: none;
}

.frb-rml-form-wrapper:after {
    border-bottom-color: var(--wmui-base100);
    border-width: 10px;
    margin-left: -10px;
}

.frb-rml-form-wrapper:before {
    border-bottom-color: var(--wmui-base70);
    border-width: 11px;
    margin-left: -11px;
}


/* styles for bottom fixed banner */
.frb-fixed .frb-rml-form-wrapper:after,
.frb-fixed .frb-rml-form-wrapper:before {
    top: 100%;
    bottom: auto;
}

.frb-fixed .frb-rml-form-wrapper:after {
    border-top-color: var(--wmui-base100);
    border-bottom-color: transparent;
}

.frb-fixed .frb-rml-form-wrapper:before {
    border-top-color: var(--wmui-base70);
    border-bottom-color: transparent;
}

.frb-rml-form input {
    width: 100%;
    padding: 6px 8px 7px;
    border: 1px solid var(--wmui-base50);
    border-radius: 2px;
    color: var(--wmui-base0);
    font-size: 14px;
    height: 45px;
}

#frb-rml-email.frb-haserror {
    border-color: var(--frb-error) !important;
    box-shadow: inset 0 0 0 1px var(--frb-error) !important;
}

.frb-error-invalidemail {
    margin: 2px 0 0 !important;
}

.frb-rml-ty {
    text-align: left;
    font-size: 14px;
}

.frb-rml-displayed .frb-rml-form-wrapper {
    position: relative;
    z-index: 10;
    display: table !important;
    margin: -4px auto 0;
    padding: 0 12px;
    width: 100%;
    max-width: 340px;
    background: transparent;
    border: none;
    box-shadow: 0 0 0 0 rgba(0, 0, 0, 0);
}

.frb-rml-displayed .frb-rml-form-wrapper:after,
.frb-rml-displayed .frb-rml-form-wrapper:before {
    display: none !important;
}

.frb-rml-displayed .frb-rml {
    display: block !important;
    margin-top: 0;
}

.frb-rml-displayed .frb-rml-form legend {
   font-size: 12px;
   line-height: 1; /*12px @12px*/
   padding-bottom: 4px;
}

@media all and (min-width: 1200px) {
    .frb-rml-displayed .frb-rml-form legend {
       font-size: 14px;
       line-height: 1.2142857143; /*17px @14px*/
    }
}

.frb-rml-displayed .frb-rml-form input {
    display: inline;
    vertical-align: middle;
    width: 200px;
    height: 45px;
    padding: 7px 8px;
    margin: 0;
    border: 1px solid var(--wmui-base50);
    border-radius: 2px;
    color: var(--frb-body);
    direction: ltr;
    line-height: 1;
}

.frb-rml-displayed .frb-rml-form .frb-btn-submit {
    display: inline;
    vertical-align: middle;
    width: auto;
    height: 45px;
    margin-top: 0;
    margin-left: 2px;
    line-height: 1;
    padding: 9px 14px;
    font-size: 14px;
    border-radius: 2px;
}

.frb-prevent-page-jump {
    display: none;
}

/* -- Submit/&quot;Donate now&quot; button -- */

.frb .frb-submit {
    height: 52px;
    display: inline-block;
    cursor: default;
    margin: 4px 5px 0;
    padding: 5px 6px;
    width: calc(100% - 9px);
    background-color: var(--wmui-base100);
    border: 1px solid var(--wmui-base70);
    border-radius: 2px;
    color: rgba(84,89,93,0.2);
    font-weight: bold;
    transition: background-color 0.5s ease;
    word-break: keep-all; /* T259535 */
    line-height: 1.3;
}

.frb .frb-submit.active {
    background-color: var(--frb-submit);
    border-color: var(--frb-submit);
    color: var(--wmui-base100);
    cursor: pointer;
    opacity: 1;
}

.frb .frb-submit.active:hover {
    background-color: var(--frb-submit-hover);
    border-color: var(--frb-submit-hover);
}

.frb-submit-amount {
    display: none;
}
.frb-submit-label-monthly {
    display: none;
}
.frb-submit-label-now {
    display: inline;
}
.form-monthly .frb-submit-label-monthly {
    display: inline;
}
.form-monthly .frb-submit-label-now {
    display: none;
}
.frb-payment-options {
    display: flex;
    flex-wrap: wrap;
    width: 100%;
}

.frb-payment-options .frb-button {
    min-width: 33%;
}

/* For 4 payment options, one row */
.frb-payment-options .frb-button:first-child:nth-last-child(4),
.frb-payment-options .frb-button:first-child:nth-last-child(4) ~ .frb-button {
    min-width: 25%;
}

/* If there are 5 buttons, make the first one of the wider ones */
.frb-payment-options .frb-button:first-child:nth-last-child(5) {
    min-width: 50%;
}

.frb-methods .frb-button,
.frb-optin .frb-button {
    flex: 1 0 0;
    padding: 5px;
}

.frb-label {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 52px;
    padding: 0 4px;
    color: var(--frb-link);
    background-color: var(--frb-button);
    border: 1px solid var(--frb-button-border);
    border-radius: 2px;
    text-align: center;
    font-weight: bold;
    transition: all 0.5s ease;
    cursor: pointer;
}

.frb-pm-cc .frb-label {
    padding: 0 2px;
}

.frb-label:hover,
.frb-rml-email:hover {
    background-color: var(--frb-button-hover);
}

.form-monthly .no-monthly .frb-label:hover {
    background-color: var(--frb-button-hover);
}

.frb-radio:checked + .frb-label {
    background-color: var(--frb-button-selected);
    border-color: var(--frb-button-border-selected);
    color: var(--wmui-base100);
}

.frb-radio:focus + .frb-label,
#frb-rml-email:focus {
    box-shadow: inset 0 0 0 1px var(--frb-radio);
    border-color: var(--frb-radio);
}

.frb-radio:focus:checked + .frb-label {
    box-shadow: inset 0 0 0 1px var(--frb-button-selected), inset 0 0 0 2px var(--wmui-base100);
}

.frb-radio:checked + .frb-label .frb-logo-payments--paypal path,
.frb-radio:checked + .frb-label .frb-logo-payments--paypal-usd path,
.frb-radio:checked + .frb-label .frb-logo-payments--amazon path,
.frb-radio:checked + .frb-label .frb-logo-payments--applepay path,
.frb-radio:checked + .frb-label .frb-logo-payments--venmo path {
    fill: var(--wmui-base100);
}

/* Error messages */
.frb-error {
    display: none;
    margin: 5px 0 5px 5px;
    font-size: 14px;
    font-weight: 500;
    line-height: 1.3;
    color: var(--frb-error);
}

.frb-form-wrapper fieldset.frb-haserror .error-highlight,
.frb-form-wrapper fieldset.frb-haserror legend::before {
    color: var(--frb-error);
    font-weight: bold;
}

/* STEP 2 UPSELL*/

.frb-upsell,
.frb-step-monthly-diff-amt .frb-amt-monthly {
    transition: background-color 0.5s ease;
    padding: 10px 4px;
    text-align: center;
}

.frb-step-monthly-diff-amt .frb-amt-monthly {
    display: block;
    padding: 0 4px 10px 4px;
}

.frb-upsell-cta,
.frb-upsell-ty {
    font-size: 17px;
    line-height: 1.3;
    font-weight: bold;
    text-align: center;
}

.frb-upsell-color,
.frb-step-monthly-diff-amt .frb-amt-monthly label {
    display: block;
    font-size: 15px;
    line-height: 1.3;
    font-weight: normal;
    margin: .5em 0;
}

.frb .frb-monthly-diff-amt-link {
    font-size: 15px;
    line-height: 1.3;
    color: var(--frb-link);
    margin: 8px 2px;
    padding: 12px 10%;
    text-align: center;
    cursor: pointer;
    font-weight: bold;
}

#frb-amt-monthly-other-input {
    position: relative;
    text-align: center;
    font-size: 18px;
    height: 45px;
}

/* Steps */
#frb-form .frb-step {
    position: absolute;
    top: 0;
    padding-top: 24px;
    width: 300px;
    margin-left: 100%; /* Start hidden */
    visibility: hidden; /* Prevent tabbing to inputs */
}
body.rtl #frb-form .frb-step {
    margin-left: 0;
    margin-right: 100%;
}

#frb-form .frb-step-1 {
    position: relative;
    margin-left: 0;
    padding-top: 0;
    visibility: visible;
}
body.rtl #frb-form .frb-step-1 {
    margin-right: 0;
}
#frb-form .frb-step-optin,
#frb-form .frb-step-upsell {
    padding-top: 45px;
}

/*
    Already Donated Modal
*/
.modal {
    display: none;
    position: absolute;
    z-index: 999;
    background: rgba(255,255,255,0.95);
    min-width: 100%;
    min-height: 100%;
    top: 0;
    left: 0;
}
.modal-container {
    width: 100%;
    max-width: 600px;
    margin: 0 auto;
    box-shadow: 0 0 10px rgba(0,0,0,0.15);
    color: #000;
    background-color: #FFF;
    padding: 40px;
    box-sizing: border-box;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%,-50%);
}
.modal-close-x {
    border: none;
    background: none;
    margin: 0;
    padding: 0;
    cursor: pointer;
    position: absolute;
    top: 24px;
    left: 24px;
    width: 22px;
    height: 18px;
    opacity: 0.5;
}

.modal-title-text {
    font-size: 17px;
    color: #262626;
    font-weight: bold;
    display: inline-block;
}
.modal-text {
    font-size: 15px;
    margin: 20px auto 0;
    line-height: 1.44;
    max-width: 530px;
}
.frb-modal-buttons {
    display: flex;
    justify-content: center;
    margin: 48px auto 0;
}
.modal-close-button {
    margin: 0 8px;
    padding: 10px 15px;
    display: block;
    border-radius: 2px;
    border: solid 1px #a2a9b1;
    background-color: #f8f9fa;
    width: 165px;
    font-family: Arial;
    font-size: 16px;
    font-weight: bold;
    color: #007de2;
    cursor: pointer;
}
.modal-close-button:hover{
    background-color: #eaf3ff;
}

/*
    Responsive
*/
@media (max-width: 1200px) {
    .frb-footer {
        display: block;
    }
    .frb-footer-message {
        display: block;
        padding-right: 0;
        max-width: 100%;
    }
    .frb-footer-button {
        display: block;
        margin: 16px auto 0;
        width: auto;
        text-align: center;
    }
    .frb-already-donated-text {
        font-size: 14px;
    }

}


/* wmf identity */

.frb-wmf-identity {
    position: absolute;
    bottom: 0;
    right: 50%;
    width: 100%;
    max-width: 100%;
    clear: both;
    color: var(--wmui-base100);
    display: flex;
    justify-content: center;
    transform: translateX(50%);
}

.frb-wmf-identity-logo {
    max-width: 100px;
    flex: 0 0 100px;
    display: flex;
    align-items: center;
    margin: 0;
}

.frb-wmf-identity-logo img {
    width: 100%;
    max-width: 100%;
    filter: invert(1);
}

@media (max-width: 859px) {
    .frb-wmf-identity {
        flex-direction: column;
        align-items: center;
        justify-content: flex-end;
    }
    .frb-wmf-identity-logo {
        flex: 0 0 auto;
    }
}

.frb-wmf-identity-message {
    flex: 0 1 auto;
    font-size: 14px;
    display: flex;
    align-items: center;
    padding-left: 20px;
    color: var(--wmui-base100);
}

.frb-wmf-identity-message p {
    margin: 0;
    font-size: 13px;
}

/* screen reader visibility */
.sr-only {
    border: 0 !important;
    clip: rect(1px, 1px, 1px, 1px) !important;
    -webkit-clip-path: inset(50%) !important;
        clip-path: inset(50%) !important;
    height: 1px !important;
    margin: -1px !important;
    overflow: hidden !important;
    padding: 0 !important;
    position: absolute !important;
    width: 1px !important;
    white-space: nowrap !important;
}

.frb-submit-txt-donate { display: none; }
.frb-submit-txt-once {display: inline; }
.frb-submit-txt-monthly { display: none; }
.form-monthly .frb-submit-txt-once { display: none; }
.form-monthly .frb-submit-txt-monthly { display: inline; }

.frb-icon-heart path {
    fill: #900;
}






    

        
            
        

        
            Thank you, dear donor!
        

        
            Your generosity helps keep Wikipedia and its sister sites thriving. Select &quot;hide appeals&quot; to suppress fundraising messages in this browser for a week, or go back to the appeal if you're still interested in donating.
        

        
            Hide appeals for a week
            Back to appeal
        

    




    

        
            
                
                Wikipedia still can't be sold.
                December 18: An important update for readers in the United States
            
            
                Please don't close this, it’s just a 1-minute read. We're sorry to interrupt, but it's Monday, December 18, and it will soon be too late to help us in our year-end fundraiser. We ask you to reflect on the number of times you visited Wikipedia this year and if you're able to give $2.75 to the Wikimedia Foundation. If everyone reading this gave just $2.75, we'd hit our goal in a few hours.

                In the age of AI, access to verifiable facts is crucial. Wikipedia matters more than ever as a reliable source for emerging technologies, and you. Your gift supports how readers use Wikipedia now, and how revolutionary new systems will utilize it tomorrow.

                Reflect on Wikipedia's usefulness in your life, and if the knowledge you gained was valuable, please give $2.75. Every contribution helps: every edit, every gift counts.
            

            
               
                   
               
               
                   Proud host of Wikipedia and its sister sites
               
            
        

        

            

                

                    

                    
                        
                            How often would you like to donate?
                        
                        
                            
                                
                                One time
                            
                            
                                
                                Give monthly
                            
                        
                    

                    
                        
                            Please select an amount (USD)
                            The average donation in the United States is around $13.
                        
                        
                            
                                
                                $2.75
                            
                            
                                
                                $10
                            
                            
                                
                                $15
                            
                            
                                
                                $25
                            
                            
                                
                                $50
                            
                            
                                
                                $75
                            
                            
                                
                                $100
                            
                            
                                Other amount
                                
                                
                                Other
                            
                        
                        
                            
                            I'll generously add a little to cover the transaction fees so you can keep 100% of my donation.
                        
                    

                    
                        
                            Please select a payment method
                        
                        

                            

                            
                                
                                
                                    Credit/Debit Card
                                    
                                        Visa

                                        MasterCard

                                        Amex

                                        JCB

                                        Discover
                                    
                                
                            

                            

                            

                            
                                
                                
                                    PayPal
                                
                            

                            
                                
                                
                                    
                                        Venmo
                                        
                                    
                                
                            

                            

                            
                                
                                
                                    
                                        Google Pay
                                        
                                    
                                
                            

                            

                        
                    

                    
                        
                            
                                Continue
                                
                                    Donate  one time
                                    Donate  monthly
                                
                            
                        
                    

                    Please select an amount (minimum $1)
                    We cannot accept donations greater than 25000 USD through our website. Please contact our major gifts staff at benefactors@wikimedia.org.
                    Please select a payment method

                    
                        Maybe later
                    

                

                

                    

                    
                        
                            Can we follow up and let you know if we need your help again? The support and advice we get from donors in the United States is priceless, but many donors don't let us stay in touch. Will you commit today, this Monday, to staying in touch with the Wikimedia Foundation?
                        
                        
                            
                                
                                Yes
                            
                            
                                
                                No
                            
                        
                        
                            Sorry to hear that. We don't email often; would you consider changing your mind?
                            Thanks for changing your mind! We’ll respect your inbox.
                        
                        
                            Your information is handled in accordance with our donor privacy policy, and each email you receive will include easy unsubscribe options.
                        
                    

                    
                        Continue
                    

                    Please select an email option

                

                

                    

                    
                        
                            
                            Almost done: Please, make it  monthly.
                        Monthly support is the best way to ensure that Wikipedia keeps thriving.
                    

                    

                        
                            No thanks! I'll make a one-time donation of 
                        

                        
                            Yes, I'll donate  each month
                        

                    

                    Yes, I'll donate monthly, but for a different amount

                

                

                    

                    Thank you for your support!
                    
                        Enter your monthly donation amount
                        
                    
                    Please select an amount (minimum $1)
                    We cannot accept donations greater than 25000 USD through our website. Please contact our major gifts staff at benefactors@wikimedia.org.
                    
                        Donate  monthly
                    

                

            

            

        

                
                    
                        Thank you! We will send you a reminder email.
                    
                    
                        
                        
                        
                        
                        
                        
                        
                        

                        Send me an email reminder
                        
                        Submit

                        
                            Please enter a valid email address i.e. name@domain.com
                        

                    
                    
                        ← Back
                    

                    
                        
                            Close
                            
                                
                                    
                                
                            
                        
                    
                
            

        
            
Problems donating? |
Other ways to give |
Frequently asked questions |
We never sell your information. By submitting, you are agreeing to our donor privacy policy and to sharing your information with the Wikimedia Foundation and its service providers in the U.S. and elsewhere.
We never sell your information. By submitting, you are agreeing to our donor privacy policy. The Wikimedia Foundation is a nonprofit, tax-exempt organization.
We never sell your information. By submitting, you are agreeing to our donor privacy policy and to sharing your information with the Wikimedia Foundation and its service providers in the U.S. and elsewhere. The Wikimedia Foundation is a nonprofit, tax-exempt organization.
If you make a recurring donation, you will be debited by the Wikimedia Foundation until you notify us to stop. We’ll send you an email which will include a link to easy cancellation instructions.

            
                
                    I already donated
                
            
        

    




    
        
            
                
                Sorry to interrupt, but time will soon run out for you to give in 2023. Please, donate $2.75.
            
        
        
            
            
                No, but maybe later when I have more time
                Yes, I'll donate $2.75
            
        
    



var frb = frb || {};

frb.HIDE_DURATION_CLOSE = 3600; // 1 hour
frb.HIDE_DURATION_RML = 604800; // 1 week

frb.show = function() {
    $('.frb-in-article').css('display', 'block');
};

frb.hide = function() {
    /* Hide the banner, and remove related event handlers */
    $('.frb').hide();
    $('.frb-prevent-page-jump').hide();
    $(window).off('.frb');
    $('#toc a').off('.frb');
};

/**
 * Show the given step
 * Literally just move into place and make it visible,
 * there may be other stuff needed to prepare it.
 *
 * @param  {string} step - name of step
 */
frb.showStep = function( step ) {

    var d = frb.reduceMotion ? 0 : 300, // animation duration in ms
        posPrev, posNext, posCrnt;

    if ( $('body').hasClass('rtl') ) {
        posPrev = { 'margin-right': '-110%' };
        posNext = { 'margin-right': '100%' };
        posCrnt = { 'margin-right': 0 };
    } else {
        posPrev = { 'margin-left': '-110%' };
        posNext = { 'margin-left': '100%' };
        posCrnt = { 'margin-left': 0 };
    }

    function movePrev( $el ) {
        $el.animate( posPrev, d, function() {
            $(this).css({ 'visibility': 'hidden' });
        });
    }

    function moveNext( $el ) {
        $el.animate( posNext, d, function() {
            $(this).css({ 'visibility': 'hidden' });
        });
    }

    function moveCrnt( $el ) {
        $el.css({ 'visibility': 'visible' }).animate( posCrnt, d );
    }

    if ( step === '1' ) {
        moveCrnt( $('.frb-step-1') );
        moveNext( $('.frb-step-optin, .frb-step-upsell, .frb-step-monthly-diff-amt') );
    } else if ( step === 'optin' ) {
        movePrev( $('.frb-step-1') );
        moveCrnt( $('.frb-step-optin') );
        moveNext( $('.frb-step-upsell, .frb-step-monthly-diff-amt') );
    } else if ( step === 'upsell' ) {
        movePrev( $('.frb-step-1, .frb-step-optin') );
        moveCrnt( $('.frb-step-upsell') );
        moveNext( $('.frb-step-monthly-diff-amt') );
    } else if ( step === 'monthly-diff-amt' ) {
        movePrev( $('.frb-step-1, .frb-step-optin, .frb-step-upsell') );
        moveCrnt( $('.frb-step-monthly-diff-amt') );
    } else {
        // console.log( 'Invalid step: ' + step );
    }

};

frb.toggleMonthly = function( monthly ) {
    if( monthly.type === 'checkbox' ){
        monthly = monthly.checked;
    }
    if ( monthly ) {
        $('#frb-frequency-monthly').prop(&quot;checked&quot;, true);
        $('#frb-monthly-checkbox').prop(&quot;checked&quot;, true);
        $('#frb-form').addClass('form-monthly');
        $('.no-monthly input[type=radio]').attr('disabled', true);
        $('.no-monthly').prop('disabled', false);
        $('#frb-form').addClass('form-monthly');
        if( $( '.form-monthly .no-monthly input[type=radio]' ).is(':checked') ) {
            $('.form-monthly .no-monthly input[type=radio]').removeAttr('checked');
            frb.setMethod({});
        }
        $('.frb-cta-label-monthly').show();
    } else {
        $('#frb-frequency-onetime').prop(&quot;checked&quot;, true);
        $('#frb-monthly-checkbox').prop(&quot;checked&quot;, false);
        $('#frb-form').removeClass('form-monthly');
        $('.no-monthly input[type=radio]').attr('disabled', false);
        $('.frb-cta-label-monthly').hide();
    }
};

frb.rml = {

    post: function() {
        /* Create the iframe for the form and use it as the form's target */
        var frameName = 'remindFrame';
        var $form = $('#frb-rml-form');
        if ( $(&quot;iframe[name=&quot; + frameName + &quot;]&quot;).length === 0 ) {
            var $iframe = $('&lt;iframe style=&quot;display: none;&quot; name=&quot;' + frameName + '&quot;>&lt;/iframe>');
            $form.attr('target', $iframe.attr('name'));
            $form.after($iframe);
        }
        $form[0].submit();
    },

    getCurrentDate: function() {
        /* Get current date in correct format for Silverpop */
        var today = new Date();
        var dd = today.getDate();
        var mm = today.getMonth()+1; // January is 0!
        var yyyy = today.getFullYear();

        if( dd &lt; 10 ) {
            dd = '0' + dd;
        }
        if( mm &lt; 10 ) {
            mm = '0' + mm;
        }

        return mm+'/'+dd+'/'+yyyy;
    },

    init: function() {
        /* Prep the reminder form */

        var form = document.getElementById('frb-rml-form');

        form.rml_country.value    = mw.centralNotice.data.country;
        form.rml_language.value   = mw.centralNotice.data.uselang;
        form.rml_submitDate.value = frb.rml.getCurrentDate();
        form.rml_segment.value    = Math.floor((Math.random() * 100) + 1);

        $('.frb-rml-link').click(function() {
            form.rml_source.value = 'B2324_121810_en6C_dsk_p1_lg_txt_169C';
            if ($('.frb').hasClass('frb-fixed')) {
                $('.frb-rml').css( 'top', $('.frb-rml-link').position().top - 195 );
            }
            else {
                $('.frb-rml').css( 'top', $('.frb-rml-link').position().top + 40 );
            }
            $('.frb-rml-form-wrapper').toggle();
            $('#frb-rml-email').focus();
            $('.frb-rml-close-wrapper').show();
            return false;
        });

        // Move the rml popup if it's open when PTF or error gets shown
        $('.frb-amounts input').on( 'click input change', function() {
            $('.frb-rml').css( 'top', $('.frb-rml-link').position().top + 40 );
            setTimeout( function() {
                $('.frb-rml').css( 'top', $('.frb-rml-link').position().top + 40 );
            }, 400 );
        });

        $('#frb-rml-submit').click(function() {
            if ( mw.util.validateEmail( form.Email.value ) ) {
                frb.rml.post();
                $('.frb-rml-form-wrapper form').hide();
                $('.frb-rml-ty').show();
                window.setTimeout( function() {
                    frb.hide();
                }, 2500);
                frb.altSetHideCookie( 'close', frb.HIDE_DURATION_RML );
                return false;
            } else {
                $('#frb-rml-email').addClass('frb-haserror').focus();
                $('.frb-error-invalidemail').show();
                return false;
            }
        });
    }

};

frb.showSidebarTooltip = function () {
    mw.loader.using( [ 'oojs-ui-core', 'mediawiki.Uri' ] ).done( function () {

        let menuPinned;
        if ( mw.config.get('skin') === 'vector-2022' ) {
            menuPinned = $('#vector-main-menu-pinned-container > #vector-main-menu').length > 0;
        } else {
            menuPinned = true; // sidebar always visible on Legacy Vector
        }

        let $donateLink = $( '#n-sitesupport a' );
        $donateLink.attr( 'href', ( i, oldUrl ) => {
            let uri = new mw.Uri( oldUrl );
            uri.query.utm_source = 'tooltipOnBannerClose';
            return uri.toString();
        });

        let popup = new OO.ui.PopupWidget( {
            $content: $( '&lt;p>You can donate at any time from this menu.&lt;/p>' ),
            padded: true,
            autoclose: true,
            align: 'forwards',
            autoFlip: false,
            $floatableContainer: menuPinned ? $donateLink : $( '#vector-main-menu-dropdown' ),
            position: menuPinned ? 'after' : 'below',
        } );

        popup.$element.css('z-index', 5); // Fix so it shows above header
        $( document.body ).append( popup.$element );
        popup.toggle( true );

        setTimeout( () => {
            popup.$element.fadeOut( frb.fadeDuration );
        }, 5000 );
    } );
};

/* jshint maxerr: 600 */
frb.amounts = frb.amounts || {};

// Hard minimum amounts that can be given
// From https://github.com/wikimedia/wikimedia-fundraising-SmashPig/blob/master/PaymentData/ReferenceData/CurrencyRates.php
// Updated 2023-01-27
frb.amounts.minimums = {
    'USD' : 1,
    'CAD' : 1.35,
    'AUD' : 1.45,
    'NZD' : 1.56,
    'GBP' : 0.81,
    'EUR' : 0.92,
    'DKK' : 6.88,
    'HUF' : 365,
    'ILS' : 3.40,
    'INR' : 10, // T309818
    'JPY' : 128,
    'MYR' : 4.31,
    'NOK' : 9.92,
    'PLN' : 4.36,
    'CZK' : 22,
    'RON' : 4.55,
    'SEK' : 10,
    'UAH' : 37,
    'ZAR' : 17,
    // Latin America
    'BRL' : 5.19,
    'ARS' : 183,
    'CLP' : 825,
    'COP' : 4684,
    'MXN' : 19,
    'PEN' : 3.80,
    'UYU' : 39,
    'CHF' : 0.92
};

frb.amounts.options7 = {
    // Big English
    'USD' : [2.75, 10, 15, 25, 50, 75, 100],
    'CAD' : [2.75, 10, 15, 25, 50, 75, 100],
    'AUD' : [2.75, 10, 15, 25, 50, 75, 100],
    'NZD' : [2.75, 10, 15, 25, 50, 75, 100],
    'GBP' : [2, 10, 15, 25, 50, 75, 100],
    'EUR' : {
        'default' : [2, 10, 15, 25, 50, 75, 100]
    },
    // Others
    'DKK' : [20, 100, 150, 200, 300, 500, 750],
    'HUF' : [500, 1000, 2000, 4000, 5000, 7000, 10000],
    'ILS' : [10, 35, 50, 100, 200, 300, 400],
    'INR' : [25, 300, 500, 1000, 1500, 3000, 5000],
    'JPY' : [300, 1000, 1500, 2000, 3000, 5000, 10000],
    'MYR' : [10, 30, 50, 100, 200, 300, 500],
    'NOK' : [20, 100, 150, 200, 500, 750, 1000],
    'PLN' : [10, 20, 50, 100, 200, 300, 500],
    'CZK' : [50, 100, 250, 500, 1000, 1500, 2500],
    'RON' : [10, 50, 75, 100, 200, 300, 500],
    'SEK' : [30, 50, 100, 200, 300, 500, 1000],
    'UAH' : [50, 75, 150, 300, 500, 750, 1000],
    'ZAR' : [30, 50, 100, 200, 300, 500, 1000],
    // Latin America
    'BRL' : [10, 20, 30, 50, 100, 200, 300],
    'ARS' : [200, 250, 500, 750, 1000, 1500, 2000],
    'CLP' : [2000, 3000, 5000, 10000, 20000, 30000, 50000],
    'COP' : [10000, 15000, 25000, 50000, 100000, 150000, 200000],
    'MXN' : [40, 70, 150, 250, 500, 700, 1000],
    'PEN' : [10, 15, 25, 50, 100, 150, 200],
    'UYU' : [100, 200, 300, 500, 1000, 1500, 2000],
    'CHF' : [3, 5, 10, 25, 50, 100, 200]
};

// 5 amount options. Since 2020 6C, no longer used
frb.amounts.options5 = {
    // Big English
    'USD' : [2.75, 15, 20, 50, 100],
    'CAD' : [2.75, 15, 20, 50, 100],
    'AUD' : [2.75, 15, 20, 50, 100],
    'NZD' : [2.75, 15, 20, 50, 100],
    'GBP' : [2, 10, 20, 50, 100],
    'EUR' : [2, 10, 20, 50, 100],
    // Others
    'DKK' : [20, 100, 200, 500, 1000],
    'HUF' : [500, 2500, 4000, 7000, 10000],
    'ILS' : [10, 50, 200, 600, 1000],
    'INR' : [150, 500, 1000, 3000, 5000],
    'JPY' : [300, 1500, 2000, 5000, 10000],
    'MYR' : [10, 50, 100, 300, 500],
    'NOK' : [20, 100, 200, 500, 1000],
    'PLN' : [10, 50, 100, 300, 500],
    'RON' : [10, 50, 100, 200, 1000],
    'SEK' : [30, 100, 200, 500, 1000],
    'UAH' : [50, 150, 300, 750, 1000],
    'ZAR' : [30, 100, 200, 500, 1000],
    // Latin America
    'BRL' : [10, 30, 50, 100, 250],
    'ARS' : [60, 200, 400, 1000, 2000],
    'CLP' : [1500, 5000, 10000, 25000, 50000],
    'COP' : [7000, 20000, 50000, 150000, 200000],
    'MXN' : [35, 100, 200, 750, 1000],
    'PEN' : [10, 50, 150, 300, 700],
    'UYU' : [70, 200, 400, 1500, 2000],
    'CHF' : [3, 10, 25, 50, 100]
};

// &quot;Average&quot; donation
frb.amounts.averages = {
    'USD' : 13,
    'CAD' : 12,
    'AUD' : 11,
    'NZD' : 12,
    'GBP' : 6,
    'EUR' : 8,
    // Others
    'DKK' : 60,
    'HUF' : 2500,
    'ILS' : 50,
    'INR' : 229,
    'JPY' : 800,
    'MYR' : 50,
    'NOK' : 75,
    'PLN' : 50,
    'CZK' : 150,
    'RON' : 50,
    'SEK' : 85,
    'UAH' : 150,
    'ZAR' : 65,
    // Latin America
    'BRL' : 25,
    'ARS' : 780,
    'CLP' : 10200,
    'COP' : 35000,
    'MXN' : 140,
    'PEN' : 30,
    'UYU' : 525,
    'CHF' : 25
};

// 'If everyone gave X'. Mostly the same as first asks option.
frb.amounts.ifEveryone = {
    // Big English
    'USD' : 2.75,
    'CAD' : 2.75,
    'AUD' : 2.75,
    'NZD' : 2.75,
    'GBP' : 2,
    'EUR' : {
        'default' : 2
    },
    // Others
    'DKK' : 20,
    'HUF' : 500,
    'ILS' : 10,
    'INR' : 25,
    'JPY' : 300,
    'MYR' : 10,
    'NOK' : 20,
    'PLN' : 10,
    'CZK' : 50,
    'RON' : 10,
    'SEK' : 30,
    'UAH' : 50,
    'ZAR' : 30,
    // Latin America
    'BRL' : 10,
    'ARS' : 175,
    'CLP' : 1500,
    'COP' : 7000,
    'MXN' : 40,
    'PEN' : 10,
    'UYU' : 100,
    'CHF' : 5
};

// Minimum fee/PTF amounts. Default is 0.35.
// Updated 2018-07-05 based on Ppena's feedback
// Updated 2019-05-21 to approx 0.35 USD equivalent
frb.amounts.feeMinimums = {
    'DKK' : 2,
    'HUF' : 100,
    'ILS' : 1.2,
    'INR' : 4,
    'JPY' : 35,
    'MYR' : 1,
    'NOK' : 3,
    'PLN' : 1.35,
    'CZK' : 7.5,
    'RON' : 1.5,
    'SEK' : 3,
    'UAH' : 10,
    'ZAR' : 5,
    // Latin America
    'BRL' : 2,
    'ARS' : 32,
    'CLP' : 255,
    'COP' : 1300,
    'MXN' : 7.4,
    'PEN' : 1.3,
    'UYU' : 15.5
};

// If one-time amount &lt;= left amount, suggest right amount for monthly
// If changing these, please update spreadsheet
// https://docs.google.com/spreadsheets/d/1z36zi8EegPLAvR5FYAgwz8ywKZ50QNB82SpwpTdk-xQ/edit#gid=1258723967
frb.amounts.monthlySuggest = {
    'EUR' : [ // also GBP
        [ 1.99, 0 ],
        [ 2.35, 1.40 ],
        [ 9, 1.75 ],
        [ 12, 2 ],
        [ 15, 2.5 ],
        [ 18, 3 ],
        [ 21, 3.5 ],
        [ 24, 4 ],
        [ 27, 4.5 ],
        [ 30, 5 ],
        [ 33, 5.5 ],
        [ 36, 6 ],
        [ 39, 6.5 ],
        [ 42, 7 ],
        [ 45, 7.5 ],
        [ 48, 8 ],
        [ 51, 8.5 ],
        [ 54, 9 ],
        [ 57, 9.5 ],
        [ 60, 10 ],
        [ 63, 10.5 ],
        [ 66, 11 ],
        [ 69, 11.5 ],
        [ 72, 12 ],
        [ 75, 12.5 ],
        [ 102, 17 ],
        [ 250, 25 ],
        [ 499, 50 ],
        [ Infinity, 0 ]
    ],
    'USD' : [ // also CAD, AUD, NZD
        [ 2.74, 0 ],
        [ 9, 1.75 ],
        [ 12, 2 ],
        [ 15, 2.5 ],
        [ 18, 3 ],
        [ 21, 3.5 ],
        [ 24, 4 ],
        [ 27, 4.5 ],
        [ 30, 5 ],
        [ 33, 5.5 ],
        [ 36, 6 ],
        [ 39, 6.5 ],
        [ 42, 7 ],
        [ 45, 7.5 ],
        [ 48, 8 ],
        [ 51, 8.5 ],
        [ 54, 9 ],
        [ 57, 9.5 ],
        [ 60, 10 ],
        [ 63, 10.5 ],
        [ 66, 11 ],
        [ 69, 11.5 ],
        [ 72, 12 ],
        [ 75, 12.5 ],
        [ 102, 17 ],
        [ 250, 25 ],
        [ 499, 50 ],
        [ Infinity, 0 ]
    ],
    'JPY' : [
        [ 299, 0 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2100, 350 ],
        [ 2400, 400 ],
        [ 2700, 450 ],
        [ 3000, 500 ],
        [ 3300, 550 ],
        [ 3600, 600 ],
        [ 3900, 650 ],
        [ 4200, 700 ],
        [ 4500, 750 ],
        [ 4800, 800 ],
        [ 5100, 850 ],
        [ 5400, 900 ],
        [ 5700, 950 ],
        [ 6000, 1000 ],
        [ 6300, 1050 ],
        [ 6600, 1100 ],
        [ 6900, 1150 ],
        [ 7200, 1200 ],
        [ 7500, 1250 ],
        [ 10800, 1800 ],
        [ 18000, 3000 ],
        [ 50000, 6000 ],
        [ Infinity, 0 ]
    ],
    'SEK' : [
        [ 25, 0 ],
        [ 50, 25 ],
        [ 100, 30 ],
        [ 200, 50 ],
        [ 300, 70 ],
        [ 500, 90 ],
        [ 1000, 110 ],
        [ 2500, 250 ],
        [ 5000, 500 ],
        [ Infinity, 0 ]
    ],
    'HUF' : [
        [ 499, 0 ],
        [ 3000, 500 ],
        [ 6000, 1000 ],
        [ 9000, 1500 ],
        [ 12000, 2000 ],
        [ 18000, 3000 ],
        [ 24000, 4000 ],
        [ 30000, 5000 ],
        [ 36000, 6000 ],
        [ 42000, 7000 ],
        [ 48000, 8000 ],
        [ 54000, 9000 ],
        [ 60000, 10000 ],
        [ Infinity, 0 ]
    ],
    'ILS' : [
        [ 9, 0 ],
        [ 10, 5 ],
        [ 60, 10 ],
        [ 90, 15 ],
        [ 120, 20 ],
        [ 180, 30 ],
        [ 240, 40 ],
        [ 300, 50 ],
        [ 360, 60 ],
        [ 420, 70 ],
        [ 480, 80 ],
        [ 540, 90 ],
        [ 600, 100 ],
        [ Infinity, 0 ]
    ],
    'ZAR' : [
        [ 29, 0 ],
        [ 30, 20 ],
        [ 50, 30 ],
        [ 100, 40 ],
        [ 300, 50 ],
        [ 450, 75 ],
        [ 600, 100 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2400, 400 ],
        [ 3000, 500 ],
        [ 3600, 600 ],
        [ Infinity, 0 ]
    ],
    'MYR' : [ // Also RON, PLN
        [ 9, 0 ],
        [ 30, 5 ],
        [ 50, 10 ],
        [ 120, 20 ],
        [ 180, 30 ],
        [ 240, 40 ],
        [ 300, 50 ],
        [ 360, 60 ],
        [ 420, 70 ],
        [ 480, 80 ],
        [ 540, 90 ],
        [ 600, 100 ],
        [ 900, 150 ],
        [ Infinity, 0 ]
    ],
    'DKK' : [ // Also NOK
        [ 19, 0 ],
        [ 20, 10 ],
        [ 120, 20 ],
        [ 180, 30 ],
        [ 300, 50 ],
        [ 450, 75 ],
        [ 600, 100 ],
        [ 750, 125 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2100, 350 ],
        [ Infinity, 0 ]
    ],
    'CZK' : [
        [ 49, 0 ],
        [ 180, 30 ],
        [ 300, 50 ],
        [ 600, 100 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2100, 350 ],
        [ 2400, 400 ],
        [ 3000, 500 ],
        [ 3600, 600 ],
        [ 4200, 700 ],
        [ 4800, 800 ],
        [ Infinity, 0 ]
    ],
    'CLP' : [
        [ 1999, 0 ],
        [ 2300, 1000 ],
        [ 2700, 1100 ],
        [ 3300, 1200 ],
        [ 4200, 1300 ],
        [ 5500, 1400 ],
        [ 9000, 1500 ],
        [ 10500, 1700 ],
        [ 16000, 2600 ],
        [ 20800, 3400 ],
        [ 26000, 4200 ],
        [ 31200, 5000 ],
        [ 38400, 6400 ],
        [ 55000, 8500 ],
        [ Infinity, 0 ]
    ],
    'UYU' : [
        [ 99, 0 ],
        [ 120, 50 ],
        [ 170, 65 ],
        [ 220, 70 ],
        [ 320, 75 ],
        [ 480, 85 ],
        [ 520, 90 ],
        [ 750, 125 ],
        [ 1050, 170 ],
        [ 1350, 225 ],
        [ 1600, 250 ],
        [ 1800, 300 ],
        [ 2100, 320 ],
        [ Infinity, 0 ]
    ],
    'PEN' : [
        [ 9, 0 ],
        [ 12, 5 ],
        [ 17, 6 ],
        [ 26, 7 ],
        [ 48, 8 ],
        [ 55, 9 ],
        [ 78, 13 ],
        [ 105, 17 ],
        [ 130, 21 ],
        [ 160, 26 ],
        [ 180, 30 ],
        [ 210, 32 ],
        [ Infinity, 0 ]
    ],
    'COP' : [
        [ 9999, 0 ],
        [ 11300, 5000 ],
        [ 17000, 5200 ],
        [ 22000, 5500 ],
        [ 27000, 5800 ],
        [ 45000, 7500 ],
        [ 55000, 9000 ],
        [ 75000, 12500 ],
        [ 105000, 17000 ],
        [ 120000, 20000 ],
        [ 160000, 25000 ],
        [ 180000, 30000 ],
        [ 250000, 34000 ],
        [ Infinity, 0 ]
    ],
    'BRL' : [
        [ 9, 0 ],
        [ 12, 6 ],
        [ 22, 7 ],
        [ 35, 8 ],
        [ 45, 9 ],
        [ 55, 10 ],
        [ 80, 12 ],
        [ 105, 16 ],
        [ 160, 25 ],
        [ 210, 35 ],
        [ 270, 45 ],
        [ 320, 50 ],
        [ Infinity, 0 ]
    ],
    'MXN' : [
        [ 39, 0 ],
        [ 48, 25 ],
        [ 60, 28 ],
        [ 110, 30 ],
        [ 160, 35 ],
        [ 260, 45 ],
        [ 270, 50 ],
        [ 350, 60 ],
        [ 550, 85 ],
        [ 650, 90 ],
        [ 750, 120 ],
        [ 1500, 160 ],
        [ Infinity, 0 ]
    ]
    
};
frb.amounts.monthlySuggest.GBP = frb.amounts.monthlySuggest.EUR;
frb.amounts.monthlySuggest.CAD = frb.amounts.monthlySuggest.USD;
frb.amounts.monthlySuggest.AUD = frb.amounts.monthlySuggest.USD;
frb.amounts.monthlySuggest.NZD = frb.amounts.monthlySuggest.USD;

frb.amounts.monthlySuggest.RON = frb.amounts.monthlySuggest.MYR;
frb.amounts.monthlySuggest.PLN = frb.amounts.monthlySuggest.MYR;
frb.amounts.monthlySuggest.NOK = frb.amounts.monthlySuggest.DKK;

frb.currencyFormats = {
    'USD' : '$\t',
    'CAD' : {
        'fr' : '$\t',
        'default' : '$\t'
    },
    'AUD' : '$\t',
    'NZD' : '$\t',
    'GBP' : '£\t',
    'EUR' : {
        'en' : '€\t',
        'cy' : '€\t',
        'ga' : '€\t',
        'mt' : '€\t',
        'nl' : '€ \t',
        'lv' : '€ \t',
        'tr' : '€ \t',
        'default' : '\t €'
    },
    // Others
    'CZK' : '\t Kč',
    'DKK' : '\t kr.',
    'HUF' : '\t Ft',
    'ILS' : {
        'he' : '\t ₪',
        'yi' : '\t ₪',
        'ar' : '\t ₪',
        'default' : '₪ \t'
    },
    'INR' : '₹ \t',
    'JPY' : '¥\t',
    'MYR' : 'RM\t',
    'NOK' : '\t kr',
    'PLN' : '\t zł',
    'RON' : '\t lei',
    'SEK' : '\t kr',
    'UAH' : '₴\t',
    'ZAR' : 'R \t',
    // Latin America
    'BRL' : {
        'en' : 'R$\t',
        'default' : 'R$ \t'
    },
    'ARS' : '$\t',
    'CLP' : '$\t',
    'COP' : '$\t',
    'MXN' : '$\t',
    'PEN' : 'S/. \t',
    'UYU' : '$U \t',
    'CHF' : '\t CHF'
};

// Check in user language first, then fall back to English
frb.countryNames = {
    'af' : {
        'ZA' : 'Suid-Afrika'
    },
    'en' : {
        'US' : 'the United States',
        'CA' : 'Canada',
        'GB' : 'the UK',
        'IE' : 'Ireland',
        'AU' : 'Australia',
        'NZ' : 'New Zealand',
        'AR' : 'Argentina',
        'AT' : 'Austria',
        'BE' : 'Belgium',
        'BR' : 'Brazil',
        'CH' : 'Switzerland',
        'CL' : 'Chile',
        'CO' : 'Colombia',
        'CZ' : 'the Czech Republic',
        'DK' : 'Denmark',
        'ES' : 'Spain',
        'FR' : 'France',
        'GR' : 'Greece',
        'HK' : 'Hong Kong',
        'HU' : 'Hungary',
        'IL' : 'Israel',
        'IN' : 'India',
        'IT' : 'Italy',
        'JP' : 'Japan',
        'LU' : 'Luxembourg',
        'LV' : 'Latvia',
        'MX' : 'Mexico',
        'MY' : 'Malaysia',
        'NL' : 'the Netherlands',
        'NO' : 'Norway',
        'PE' : 'Peru',
        'PL' : 'Poland',
        'PT' : 'Portugal',
        'RO' : 'Romania',
        'SE' : 'Sweden',
        'SK' : 'Slovakia',
        'UA' : 'Ukraine',
        'UY' : 'Uruguay',
        'ZA' : 'South Africa'
    },
    'ca' : {
        'AT' : 'd’Àustria',
        'BE' : 'de Bèlgica',
        'DK' : 'de Dinamarca',
        'ES' : 'a Espanya',
        'HU' : 'd’Hongria',
        'IL' : 'd’Israel',
        'LV' : 'de Letònia',
        'LU' : 'de Luxemburg',
        'MY' : 'de Malàisia',
        'NO' : 'de Noruega',
        'PL' : 'de Polònia',
        'PT' : 'de Portugal',
        'RO' : 'de Romania',
        'SK' : 'd’Eslovàquia',
        'ZA' : 'de Sud-àfrica',
        'UA' : 'd’Ucraïna'
    },
    'cs' : {
        'CZ' : 'v České republice',
        'AT' : 'v Rakousku',
        'BE' : 'v Belgii',
        'DK' : 'v Dánsku',
        'GR' : 'v Řecku',
        'IL' : 'v Izraeli',
        'LU' : 'v Lucembursku',
        'MY' : 'v Malajsii',
        'NO' : 'v Norsku',
        'PT' : 'v Portugalsku',
        'SE' : 've Švédsku',
        'ZA' : 'v Jihoafrické republice'
    },
    'es' : {
        'AR' : 'en Argentina',
        'AT' : 'en Austria',
        'BE' : 'en Bélgica',
        'BR' : 'en Brasil',
        'CL' : 'en Chile',
        'CO' : 'en Colombia',
        'DK' : 'en Dinamarca',
        'ES' : 'en España',
        'HU' : 'en Hungría',
        'IL' : 'en Israel',
        'LU' : 'en Luxemburgo',
        'LV' : 'en Letonia',
        'MX' : 'en México',
        'MY' : 'en Malasia ',
        'NO' : 'en Noruega',
        'PE' : 'en Perú',
        'PL' : 'en Polonia',
        'PT' : 'en Portugal',
        'RO' : 'en Rumania',
        'SK' : 'en Eslovaquia',
        'UA' : 'en Ucrania',
        'US' : 'en los Estados Unidos',
        'UY' : 'en Uruguay',
        'ZA' : 'en Sudafrica'
    },
    'da' : {
        'AT' : 'i Østrig',
        'BE' : 'i Belgien',
        'CZ' : 'i Tjekkiet',
        'DK' : 'i Danmark',
        'ES' : 'i Spanien',
        'GR' : 'i Grækenland',
        'HU' : 'i Ungarn',
        'IL' : 'i Israel',
        'LV' : 'i Letland',
        'LU' : 'i Luxembourg',
        'MY' : 'i Malaysia',
        'NO' : 'i Norge',
        'PL' : 'i Polen',
        'PT' : 'i Portugal',
        'RO' : 'i Rumænien',
        'SE' : 'i Sverige',
        'SK' : 'i Slovakiet',
        'ZA' : 'i Sydafrika',
        'UA' : 'i Ukraine'
    },
    'nl' : {
        'NL' : 'in Nederland',
        'AT' : 'in Oostenrijk',
        'BE' : 'in België',
        'DK' : 'in Denemarken',
        'CZ' : 'in Tsjechië',
        'ES' : 'in Spanje',
        'GR' : 'in Griekenland',
        'HU' : 'in Hongarije',
        'IL' : 'in Israël',
        'LV' : 'in Letland',
        'LU' : 'in Luxemburg',
        'MY' : 'in Maleisië',
        'NO' : 'in Noorwegen',
        'PL' : 'in Polen',
        'PT' : 'in Portugal',
        'RO' : 'in Roemenië',
        'SE' : 'in Zweden',
        'SK' : 'in Slowakije',
        'ZA' : 'in Zuid-Afrika',
        'UA' : 'in Oekraïne'
    },
    'fr' : {
        'AT' : 'en Autriche',
        'BE' : 'en Belgique',
        'CH' : 'en Suisse',
        'CA' : 'au Canada',
        'CZ' : 'en République tchèque',
        'DK' : 'au Danemark',
        'ES' : 'en Espagne',
        'FR' : 'en France',
        'GR' : 'en Grèce',
        'HU' : 'en Hongrie',
        'IL' : 'en Israël',
        'LV' : 'en Lettonie',
        'LU' : 'au Luxembourg',
        'MY' : 'en Malaisie',
        'NO' : 'en Norvège',
        'PL' : 'en Pologne',
        'PT' : 'au Portugal',
        'RO' : 'en Roumanie',
        'SE' : 'en Suède',
        'SK' : 'en Slovaquie',
        'ZA' : 'en Afrique du Sud',
        'UA' : 'en Ukraine'
    },
    'de' : {
        'AT' : 'in Österreich',
        'BE' : 'in Belgien',
        'CH' : 'in der Schweiz',
        'CZ' : 'in Tschechien',
        'DK' : 'in Dänemark',
        'ES' : 'in Spanien',
        'GR' : 'in Griechenland',
        'HU' : 'in Ungarn',
        'IL' : 'in Israel',
        'LV' : 'in Lettland',
        'LU' : 'in Luxemburg',
        'MY' : 'in Malaysia',
        'NO' : 'in Norwegen',
        'PL' : 'in Polen',
        'PT' : 'in Portugal',
        'RO' : 'in Rumänien',
        'SE' : 'in Schweden',
        'SK' : 'in der Slowakei',
        'ZA' : 'in Südafrika',
        'UA' : 'in der Ukraine'
    },
    'el' : {
        'AT' : 'στην Αυστρία',
        'BE' : 'στο Βέλγιο',
        'CZ' : 'στην Τσεχία',
        'DK' : 'στη Δανία',
        'ES' : 'στην Ισπανία',
        'GR' : 'στην Ελλάδα',
        'HU' : 'στην Ουγγαρία',
        'IL' : 'στο Ισραήλ',
        'LV' : 'στη Λετονία',
        'LU' : 'στο Λουξεμβούργο',
        'MY' : 'στη Μαλαισία',
        'NO' : 'στη Νορβηγία',
        'PL' : 'στην Πολωνία',
        'PT' : 'στην Πορτογαλία',
        'RO' : 'στη Ρουμανία',
        'SE' : 'στη Σουηδία',
        'SK' : 'στη Σλοβακία',
        'ZA' : 'στη Νότια Αφρική',
        'UA' : 'στην Ουκρανία'
    },
    'he' : {
        'AT' : 'אוסטרליה',
        'BE' : 'בלגיה',
        'CZ' : &quot;בצ'כיה&quot;,
        'DK' : 'דנמרק',
        'ES' : 'ספרד',
        'GR' : 'ביוון',
        'HU' : 'הונגריה',
        'IL' : 'ישראל',
        'LV' : 'לטביה',
        'LU' : 'לוקסמבורג',
        'MY' : 'מלזיה',
        'NO' : 'נורווגיה',
        'PL' : 'פולין',
        'PT' : 'פורטוגל',
        'RO' : 'רומניה',
        'SE' : 'בשוודיה',
        'SK' : 'סלובקיה',
        'ZA' : 'דרום אפריקה',
        'UA' : 'אוקראינה'
    },
    'hu' : {
        'AT' : 'ausztriai',
        'BE' : 'belgiumi',
        'DK' : 'dániai',
        'ES' : 'spanyolországi',
        'HU' : 'magyarországi',
        'IL' : 'izraeli',
        'LV' : 'lettországi',
        'LU' : 'luxemburgi',
        'MY' : 'malajziai',
        'NO' : 'norvégiai',
        'PL' : 'lengyelországi',
        'PT' : 'portugáliai',
        'RO' : 'romániai',
        'SK' : 'szlovákiai',
        'ZA' : 'dél-afrikai',
        'UA' : 'ukrajnai'
    },
    'it' : {
        'IT' : 'Italia',
        'CH' : 'Svizzera'
    },
    'lv' : {
        'AT' : 'valstī Austrijā',
        'BE' : 'valstī Beļģijā',
        'DK' : 'valstī Dānijā',
        'ES' : 'valstī Spānijā',
        'HU' : 'valstī Ungārijā',
        'IL' : 'Izraēlas valstī',
        'LV' : 'valstī Latvijā',
        'LU' : 'valstī Luksemburgā',
        'MY' : 'valstī Malaizijā',
        'NO' : 'valstī Norvēģijā',
        'PL' : 'valstī Polijā',
        'PT' : 'valstī Portugālē',
        'RO' : 'valstī Rumānijā',
        'SK' : 'valstī Slovākijā',
        'ZA' : 'Dienvidāfrikas valstī',
        'UA' : 'valstī Ukrainā'
    },
    'nb' : {
        'AT' : 'i Østerrike',
        'BE' : 'i Belgia',
        'CZ' : 'i Tsjekkia',
        'DK' : 'i Danmark',
        'ES' : 'i Spania',
        'GR' : 'i Hellas',
        'HU' : 'i Ungarn',
        'IL' : 'i Israel',
        'LV' : 'i Latvia',
        'LU' : 'i Luxembourg',
        'MY' : 'i Malaysia',
        'NO' : 'i Norge',
        'PL' : 'i Polen',
        'PT' : 'i Portugal',
        'RO' : 'i Romania',
        'SE' : 'i Sverige',
        'SK' : 'i Slovakia',
        'ZA' : 'i Sør-Afrika',
        'UA' : 'i Ukraina'
    },
    'pl' : {
        'AT' : 'w Austrii',
        'BE' : 'w Belgii',
        'DK' : 'w Danii',
        'ES' : 'w Hiszpanii',
        'HU' : 'na Węgrzech',
        'IL' : 'w Izraelu',
        'LV' : 'na Łotwie',
        'LU' : 'w Luksemburgu',
        'MY' : 'w Malezji',
        'NO' : 'w Norwegii',
        'PL' : 'w Polsce',
        'PT' : 'w Portugalii',
        'RO' : 'w Rumunii',
        'SK' : 'na Słowacji',
        'ZA' : 'w Republice Południowej Afryki',
        'UA' : 'na Ukrainie'
    },
    'pt' : {
        'AT' : 'na Áustria',
        'BE' : 'na Bélgica',
        'BR' : 'no Brasil',
        'CZ' : 'na República Checa',
        'DK' : 'na Dinamarca',
        'ES' : 'na Espanha',
        'GR' : 'na Grécia',
        'HU' : 'na Hungria',
        'IL' : 'em Israel',
        'LV' : 'na Letónia',
        'LU' : 'no Luxemburgo',
        'MY' : 'na Malásia',
        'NO' : 'na Noruega',
        'PL' : 'na Polónia',
        'PT' : 'em Portugal',
        'RO' : 'na Roménia',
        'SE' : 'na Suécia',
        'SK' : 'na Eslováquia',
        'ZA' : 'na África do Sul',
        'UA' : 'na Ucrânia'
    },
    'ro' : {
        'AT' : 'din Austria',
        'BE' : 'din Belgia',
        'DK' : 'din Danemarca',
        'ES' : 'în Spania',
        'HU' : 'din Ungaria',
        'IL' : 'din Israel',
        'LV' : 'din Latvia',
        'LU' : 'din Luxemburg',
        'MY' : 'din Malaezia',
        'NO' : 'din Norvegia',
        'PL' : 'din Polonia',
        'PT' : 'din Portugalia',
        'RO' : 'din România',
        'SK' : 'din Slovacia',
        'ZA' : 'din Africa de Sud',
        'UA' : 'din Ucraina'
    },
    'ru' : {
        'AT' : 'в Австрии',
        'BE' : 'в Бельгии',
        'DK' : 'в Дании',
        'ES' : 'в Испании',
        'HU' : 'в Венгрии',
        'IL' : 'в Израиле',
        'LV' : 'в Латвии',
        'LU' : 'в Люксембурге',
        'MY' : 'в Малайзии',
        'NO' : 'в Норвегии',
        'PL' : 'в Польше',
        'PT' : 'в Португалии',
        'RO' : 'в Румынии',
        'SK' : 'в Словакии',
        'ZA' : 'в Южной Африке',
        'UA' : 'в Украине'
    },
    'sk' : {
        'AT' : 'v Rakúsku',
        'BE' : 'v Belgicku',
        'DK' : 'v Dánsku',
        'ES' : 'v Španielsku',
        'HU' : 'v Maďarsku',
        'IL' : 'v Izraeli',
        'LV' : 'v Lotyšsku',
        'LU' : 'v Luxembursku',
        'MY' : 'v Malajzii',
        'NO' : 'v Nórsku',
        'PL' : 'v Poľsku',
        'PT' : 'v Portugalsku',
        'RO' : 'v Rumunsku',
        'SK' : 'na Slovensku',
        'ZA' : 'v Juhoafrickej republike',
        'UA' : 'na Ukrajine'
    },
    'sv' : {
        'SE' : 'i Sverige',
        'AT' : 'i Österrike',
        'BE' : 'i Belgien',
        'CZ' : 'i Tjeckien',
        'DK' : 'i Danmark',
        'ES' : 'i Spanien',
        'GR' : 'i Grekland',
        'HU' : 'i Ungern',
        'IL' : 'i Israel',
        'LV' : 'i Lettland',
        'LU' : 'i Luxemburg',
        'MY' : 'i Malaysia',
        'NO' : 'i Norge',
        'PL' : 'i Polen',
        'PT' : 'i Portugal',
        'RO' : 'i Rumänien',
        'SK' : 'i Slovakien',
        'ZA' : 'i Sydafrika',
        'UA' : 'i Ukraina'
    },
    'uk' : {
        'AT' : 'у Австрії',
        'BE' : 'у Бельгії',
        'DK' : 'у Данії',
        'ES' : 'в Іспанії',
        'HU' : 'в Угорщині',
        'IL' : 'в Ізраїлі',
        'LV' : 'у Латвії',
        'LU' : 'у Люксембургу',
        'MY' : 'у Малайзії',
        'NO' : 'у Норвегії',
        'PL' : 'у Польщі',
        'PT' : 'у Португалії',
        'RO' : 'у Румунії',
        'SK' : 'у Словаччині',
        'ZA' : 'у ПАР',
        'UA' : 'в Україні'
    }
};

/* 
Most of the translations are actually using &quot;in COUNTRY&quot; or similar to account for grammar differences.
So this makes English do the same, and allows us to use a clearer %in-country% variable, while avoiding
breaking old content using %country%.
*/
frb.inCountryNames = JSON.parse( JSON.stringify( frb.countryNames ) ); // deep copy
frb.inCountryNames.en = {
    'US' : 'in the United States',
    'CA' : 'in Canada',
    'GB' : 'in the UK',
    'IE' : 'in Ireland',
    'AU' : 'in Australia',
    'NZ' : 'in New Zealand',
    'AR' : 'in Argentina',
    'AT' : 'in Austria',
    'BE' : 'in Belgium',
    'BR' : 'in Brazil',
    'CH' : 'in Switzerland',
    'CL' : 'in Chile',
    'CO' : 'in Colombia',
    'CZ' : 'in the Czech Republic',
    'DK' : 'in Denmark',
    'ES' : 'in Spain',
    'FR' : 'in France',
    'GR' : 'in Greece',
    'HK' : 'in Hong Kong',
    'HU' : 'in Hungary',
    'IL' : 'in Israel',
    'IN' : 'in India',
    'IT' : 'in Italy',
    'JP' : 'in Japan',
    'LU' : 'in Luxembourg',
    'LV' : 'in Latvia',
    'MX' : 'in Mexico',
    'MY' : 'in Malaysia',
    'NL' : 'in the Netherlands',
    'NO' : 'in Norway',
    'PE' : 'in Peru',
    'PL' : 'in Poland',
    'PT' : 'in Portugal',
    'RO' : 'in Romania',
    'SE' : 'in Sweden',
    'SK' : 'in Slovakia',
    'UA' : 'in Ukraine',
    'UY' : 'in Uruguay',
    'ZA' : 'in South Africa'
};

frb.dayNames = {
    'en' : [ 'Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday' ],
    'ca' : [ 'diumenge', 'dilluns', 'dimarts', 'dimecres', 'dijous', 'divendres', 'dissabte' ],
    'ja' : [ '日', '月', '火', '水', '木', '金', '土' ],
    'es' : [ 'domingo', 'lunes', 'martes', 'miércoles', 'jueves', 'viernes', 'sábado' ],
    'sv' : [ 'söndag', 'måndag', 'tisdag', 'onsdag', 'torsdag', 'fredag', 'lördag' ],
    'da' : [ 'søndag', 'mandag', 'tirsdag', 'onsdag', 'torsdag', 'fredag', 'lørdag' ],
    'nb' : [ 'søndagen', 'mandagen', 'tirsdagen', 'onsdagen', 'torsdagen', 'fredagen', 'lørdagen' ],
    'it' : [ 'domenica', 'lunedì', 'martedì', 'mercoledì', 'giovedì', 'venerdì', 'sabato' ],
    'nl' : [ 'zondag', 'maandag', 'dinsdag', 'woensdag', 'donderdag', 'vrijdag', 'zaterdag' ],
    'fr' : [ 'dimanche', 'lundi', 'mardi', 'mercredi', 'jeudi', 'vendredi', 'samedi' ],
    'de' : [ 'Sonntag', 'Montag', 'Dienstag', 'Mittwoch', 'Donnerstag', 'Freitag', 'Samstag' ],
    'he' : [ 'ראשון', 'שני', 'שלישי', 'רביעי', 'חמישי', 'שישי', 'שבת' ],
    'lv' : [ 'svētdienā', 'pirmdienā', 'otrdienā', 'trešdienā', 'ceturtdienā', 'piektdienā', 'sestdienā' ],
    'pl' : [ 'niedzielę', 'poniedziałek', 'wtorek', 'środę', 'czwartek', 'piątek', 'sobotę' ],
    'pt' : [ 'neste domingo', 'nesta segunda-feira', 'nesta terça-feira', 'nesta quarta-feira', 'nesta  quinta-feira', 'nesta sexta-feira', 'neste sábado' ],
    'ru' : [ 'воскресенье', 'понедельник', 'вторник', 'среду', 'четверг', 'пятницу', 'субботу' ],
    'uk' : [ 'неділі', 'понеділка', 'вівторка', 'середи', 'четверга', 'п’ятниц', 'суботи' ],
    'hu' : [ 'vasárnap', 'hétfő', 'kedd', 'szerda', 'csütörtök', 'péntek', 'szombat' ],
    'ro' : [ 'duminică', 'luni', 'marți', 'miercuri', 'joi', 'vineri', 'sâmbătă' ],
    'af' : [ 'Sondag', 'Maandag', 'Dinsdag', 'Woensdag', 'Donderdag', 'Vrydag', 'Saterdag' ],
    'aa' : [ 'Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday' ]
};

// &quot;This fooday&quot; translations. Needed for some languages where gender varies and &quot;this&quot; must agree
frb.dayNamesThis = {
    'en' : [ 'this Sunday', 'this Monday', 'this Tuesday', 'this Wednesday', 'this Thursday', 'this Friday', 'this Saturday' ],
    'el' : [ 'Αυτήν την	Κυριακή', 'Αυτή τη Δευτέρα', 'Αυτήν την	Τρίτη', 'Αυτήν την Τετάρτη', 'Αυτήν την	Πέμπτη', 'Αυτήν την Παρασκευή', 'Αυτό το Σάββατο' ],
    'jp' : [ 'この日曜日', 'この月曜日', 'この火曜日', 'この水曜日', 'この木曜日', 'この金曜日', 'この土曜日' ],
    'it' : [ 'questa domenica', 'questo lunedì', 'questo martedì', 'questo mercoledì', 'questo giovedì', 'questo venerdì', 'questo sabato'],
    'pl' : [ 'w tę niedzielę', 'w ten poniedziałek', 'w ten wtorek', 'w tę środę', 'w ten czwartek', 'w ten piątek', 'w tę sobotę' ],
    'ru' : [ 'в это воскресенье', 'в этот понедельник', 'в этот вторник', 'в эту среду', 'в этот четверг', 'в эту пятницу', 'в эту субботу' ],
    'uk' : [ 'цієї неділі', 'цього понеділка', 'цього вівторка', 'цієї середи', 'цього четверга', 'цієї п’ятниці', 'цієї суботи' ],
    'pt' : [ 'este domingo', 'esta segunda-feira', 'esta terça-feira', 'esta quarta-feira', 'esta quinta-feira', 'esta sexta-feira', 'este sábado'],
    'sk' : [ 'túto nedeľu', 'tento pondelok', 'tento utorok', 'túto stredu', 'tento štvrtok', 'tento piatok', 'túto sobotu'],
    'cs' : [ 'tuto neděli', 'toto pondělí', 'toto úterý', 'tuto středu', 'tento čtvrtek', 'tento pátek', 'tuto sobotu']
};

frb.iPadTranslations = {
    'en' : 'iPad'
};

// Insert any localize data overrides here

/* jshint maxerr: 600 */
/* MediaWiki:FundraisingBanners/CoreJS-2018.js
 * Core code for banner forms, with new inline error messages
 */

var frb = frb || {};

/**
 * Test for general ES6 support
 *
 * Checks for arrow functions, default parameters, NodeList.prototype.forEach()
 * Should be roughly Chrome 51+, Firefox 50+, Edge 16+, Safari 10+
 * Based on https://gist.github.com/bendc/d7f3dbc83d0f65ca0433caf90378cd95
 * @return {boolean}
 */
frb.supportedBrowser = function() {
    try {
        new Function('(a = 0) => a');
        document.querySelectorAll('.frb').forEach(a => a);
        return true;
    }
    catch (err) {
        return false;
    }
}();

if ( !mw.centralNotice.adminUi ) { // T262693
    frb.loadedTime = Date.now();
    frb.didSelectAmount = false;
    frb.optinRequiredCountries =
        [ 'AR', 'AT', 'BE', 'BR', 'CL', 'CO', 'CZ', 'DK', 'ES', 'FR', 'GB', 'GR', 'HU', 'IE', 'IT', 'IL',
          'LU', 'LV', 'MX', 'NL', 'NO', 'PE', 'PL', 'PT', 'RO', 'SE', 'SK', 'UA', 'UY' ];
    frb.optinRequired = frb.optinRequiredCountries.indexOf(mw.centralNotice.data.country) !== -1;
    frb.maxUSD = 25000;
    frb.reduceMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
}

// Keyboard shortcut to go from banner preview to editor - Ctrl+Shift+E
if ( mw.config.get('wgUserName') ) {
    if ( mw.config.get('wgUserName').match(/\(WMF\)/) ) {
        window.addEventListener('keydown', function(e) {
            if ( e.ctrlKey &amp;&amp; e.shiftKey &amp;&amp; e.keyCode === 69 ) {
                window.open( 'https://meta.wikimedia.org/wiki/Special:CentralNoticeBanners/Edit/' + mw.centralNotice.data.banner );
            }
        });
    }
}

/**
 * Main function to submit to paymentswiki
 *
 * @param  {Object} options
 * - method (required)
 * - submethod (optional)
 * - gateway (optional)
 * - skipValidation (optional boolean, for pp-usd. Not yet implemented.)
 * @param  {Boolean} isEndowment - deprecated, set frb.isEndowment instead
 */
frb.submitForm = function( options, isEndowment ) {

    var uri = new mw.Uri('https://payments.wikimedia.org/index.php/Special:GatewayChooser');
    var params = {};

    if ( !frb.validateForm( options ) ) {
        frb.extraData.validateError = 1; // Flag they had an error, even if fixed later
        return false; // Error, bail out of submitting
    }

    // Skip form chooser for Apple Pay / Google Pay
    if ( options.method === 'apple' || options.method === 'google' ) {
        uri = new mw.Uri('https://payments.wikimedia.org/index.php/Special:AdyenCheckoutGateway');
    }

    // Skip form chooser for Venmo
    if ( options.method === 'venmo' ) {
        uri = new mw.Uri('https://payments.wikimedia.org/index.php/Special:BraintreeGateway');
    }

    // Form selection data
    params.payment_method = options.method;
    if ( options.submethod ) {
        params.payment_submethod = options.submethod;
    }
    if ( options.gateway ) {
        params.gateway = options.gateway;
    }
    if ( options.variant ) {
        params.variant = options.variant;
    }
    params.recurring = frb.getRecurring();

    if ( params.recurring &amp;&amp; params.variant &amp;&amp; params.variant.match( /monthlyConvert/ ) ) {
        // Post-payments monthly convert makes no sense if it's already recurring
        // Avoid things like T312905
        delete params.variant;
    }

    params.currency = frb.getCurrency(mw.centralNotice.data.country) || 'USD';

    params.uselang = mw.centralNotice.data.uselang || 'en';
    params.country = mw.centralNotice.data.country || 'XX';

    if ( params.uselang === 'pt' &amp;&amp; params.country === 'BR' ) {
        params.uselang = 'pt-br';
    }
    if ( params.uselang === 'es' &amp;&amp;
        ( params.country === 'AR' || params.country === 'CL' ||
          params.country === 'CO' || params.country === 'MX' ||
          params.country === 'PE' || params.country === 'UY' ||
          params.country === 'US' )
    ) {
        params.uselang = 'es-419';
    }

    // Adyen override. frb.ccAdyenCountries is defined in LocalizeJS-2017.js
    if ( params.payment_method === 'cc' &amp;&amp; frb.ccAdyenCountries.indexOf( params.country ) !== -1 ) {
        params.gateway = 'adyen';
    }
    // dLocal override for South Africa
    if ( params.payment_method === 'cc' &amp;&amp; params.country === 'ZA' ) {
        params.gateway = 'astropay';
    }

    // Amount
    var amount = frb.getAmount();
    if ( $('#frb-ptf-checkbox').prop('checked') ) {
        amount = amount + frb.calculateFee(amount);
        frb.extraData.ptf = 1;
    }
    params.amount = amount;

    // Email optin
    if ( frb.optinRequired &amp;&amp; $('input[name=&quot;opt_in&quot;]').length > 0 ) {
        var opt_inValue = $('input[name=&quot;opt_in&quot;]:checked').val();
        params.opt_in   = opt_inValue; // frb.validateForm() already checked it's 1 or 0
    }

    // Tracking info
    if ( isEndowment || frb.isEndowment ) {
        params.utm_medium = 'endowment';
        params.appeal = 'EndowmentQuote';
    } else {
        params.utm_medium = 'sitenotice';
    }
    params.utm_campaign = mw.centralNotice.data.campaign || 'test';
    params.utm_source   = frb.buildUtmSource(params);

    frb.extraData.vw = window.innerWidth;
    frb.extraData.vh = window.innerHeight;
    frb.extraData.time = Math.round( (Date.now() - frb.loadedTime)/1000 );

    if ( navigator.brave !== undefined ) { // T283367
        frb.extraData.brave = '1';
    }

    if ( !$.isEmptyObject( frb.extraData ) ) {
        params.utm_key = frb.buildUtmKey( frb.extraData );
    }

    // Link to Banner History if enabled
    var mixins = mw.centralNotice.getDataProperty( 'mixins' );
    if ( mixins &amp;&amp; mixins.bannerHistoryLogger ) {
        params.bannerhistlog = mw.centralNotice.bannerHistoryLogger.id;
    }

    uri.extend(params);

    // Set a cookie with current location so we can return here from TY page
    mw.loader.using( [ 'mediawiki.cookie', 'mediawiki.util' ] ).then( function () {
        // Exclude URL parameters like banner, but cope with paths like /w/index.php?title=Foo
        var returnToUrl = window.location.origin + mw.util.getUrl();
        mw.cookie.set(
            'fundraising_returnTo',
            returnToUrl,
            { expires: 300, prefix: '', domain: '.wikipedia.org', secure: true }
        );
    });

    if ( mixins &amp;&amp; mixins.bannerHistoryLogger ) {
        mw.centralNotice.bannerHistoryLogger.ensureLogSent().always(function() {
            frb.goToPayments( uri );
        });
    } else {
        frb.goToPayments( uri );
    }

};

frb.goToPayments = function( uri ) {
    if ( window.top !== window.self ) {
        // banner is in a frame, open payments in a new tab
        window.open( uri.toString() );
    } else {
        window.location.href = uri.toString();
    }
};

/**
 * Check the form for errors.
 *
 * Called on submission, can also be called on input
 *
 * @param {object} options
 * @return {boolean} Whether form is error-free
 */
frb.validateForm = function( options ) {
    var error = false;

    /* Reset all errors */
    $('.frb-haserror').removeClass('frb-haserror');
    $('.frb-error').hide();

    if ( !options.method ) {
        error = true;
        $('.frb-methods').addClass('frb-haserror');
        $('.frb-error-method').show();
    }

    if ( !frb.validateAmount() ) {
        error = true;
    }

    /* Email optin */
    if ( frb.optinRequired &amp;&amp; $('.frb-optin').is(':visible') ) {
        var opt_inValue = $('input[name=&quot;opt_in&quot;]:checked').val();
        if ( opt_inValue !== '1' &amp;&amp; opt_inValue !== '0' ) {
            $('.frb-optin').addClass('frb-haserror');
            $('.frb-error-optin').show();
            error = true;
        }
    }

    return !error;
};

/**
 * Check if selected amount is valid i.e. a positive number, between minimum and maximum.
 * If not, show an error and return false.
 */
frb.validateAmount = function() {

    var amount = frb.getAmount(),
        currency  = frb.getCurrency( mw.centralNotice.data.country ),
        minAmount = frb.amounts.minimums[ currency ],
        maxAmount = Math.round( frb.maxUSD * minAmount );
        // Math.round to account for floating point math errors: https://phabricator.wikimedia.org/T246262

    if ( amount === null || isNaN(amount) || amount &lt;= 0 || amount &lt; minAmount ) {
        $('fieldset.frb-amounts').addClass('frb-haserror');
        $('.frb-error-bigamount').hide();
        $('.frb-error-smallamount').show();
        return false;
    } else if ( amount > Math.round( maxAmount ) ) {
        $('fieldset.frb-amounts').addClass('frb-haserror');
        $('.frb-error-bigamount').show();
        return false;
    } else {
        $('fieldset.frb-amounts').removeClass('frb-haserror');
        $('.frb-error-smallamount, .frb-error-bigamount').hide();
        return true;
    }
};

/**
 * Build the utm_source for analytics.
 *
 * Own function so it can be overriden for weird tests
 *
 * @param  {Object} params
 * @return {string} utm_source
 */
frb.buildUtmSource = function(params) {

    var utm_source;
    var fullDottedPaymentMethod = params.payment_method;
    if ( params.recurring ) {
        fullDottedPaymentMethod = 'r' + fullDottedPaymentMethod;
    }
    if ( params.payment_submethod ) {
        fullDottedPaymentMethod = fullDottedPaymentMethod + '.' + params.payment_submethod;
    }

    utm_source = mw.centralNotice.data.banner;

    // Keeping opt-in in utm_source for safety for now
    // Eventually remove it, or move to utm_key?
    if ( params.opt_in ) {
        utm_source += '_optIn' + params.opt_in;
    }

    utm_source += '.no-LP.' + fullDottedPaymentMethod;

    return utm_source;
};

/**
 * Build a string for utm_key from extra tracking data
 *
 * @param  {Object} data
 * @return {string} utm_key
 */
frb.buildUtmKey = function(data) {
    var dataArray = [];
    for (var key in data) {
        if (data.hasOwnProperty(key)) {
            dataArray.push( key + '_' + data[key] );
        }
    }
    return dataArray.join('~');
};

/**
 * Determine if we should show recurring choice on step 2
 * 
 * NOTE 2023-12-07: we don't currently use this for step 2, since there are no
 *	banners where users select method before frequency. However it is used by
 *	frb.shouldShowMonthlyConvert()
 *
 * @param  {Object} options     Including method and optional gateway
 * @param  {String} country
 * @return {boolean}
 */
frb.shouldShowRecurring = function( options, country ) {

    if ( frb.isEndowment ) {
        return false;
    }
    if ( frb.noRecurringCountries.indexOf( country ) !== -1 ) { // Defined in LocalizeJS-2017.js
        return false;
    }
    if ( options.method === undefined ) {
        return true; // Show if a method hasn't been selected yet
    }
    if ( [ 'cc', 'paypal', 'venmo', 'apple', 'google' ].indexOf( options.method ) !== -1 ) {
        return true;
    }
    // Adyen iDEAL
    if ( options.submethod === 'rtbt_ideal' ) {
        return true;
    }
    if ( options.submethod === 'upi' || options.submethod === 'paytmwallet' ) {
        return true;
    }
    return false;
};

/* Is recurring method selected? This function can be overriden for different forms */
frb.getRecurring = function() {
    // Can't use simple form.frequency.value, doesn't work in IE
    var selected = $('#frb-form input[name=&quot;frequency&quot;]:checked').val();
    return selected === 'monthly';
};

/* Return amount selected */
frb.getAmount = function() {
    var form = document.getElementById('frb-form');
    var amount = null;
    frb.extraData.otherAmt = 0;

    // If there are some amount radio buttons, then look for the checked one
    if (form.amount) {
        for (var i = 0; i &lt; form.amount.length; i++) {
            if (form.amount[i].checked) {
                amount = form.amount[i].value;
            }
        }
    }

    // Check the &quot;other&quot; amount box
    if (form.otherAmount.value !== '') {
        var otherAmount = form.otherAmount.value;
        otherAmount = otherAmount.replace(/[,.](\d)$/, ':$10');
        otherAmount = otherAmount.replace(/[,.](\d)(\d)$/, ':$1$2');
        otherAmount = otherAmount.replace(/[$£€¥,.]/g, '');
        otherAmount = otherAmount.replace(/:/, '.');
        amount = otherAmount;
        frb.extraData.otherAmt = 1;
    }

    amount = parseFloat(amount);

    if ( isNaN(amount) ) {
        return 0;
    } else {
        return amount;
    }

};

/* Localize the amount errors. Call when initialising banner. */
frb.localizeErrors = function() {
    var currency  = frb.getCurrency( mw.centralNotice.data.country ),
        language = mw.centralNotice.data.uselang,
        minAmount = frb.amounts.minimums[ currency ],
        maxAmount = Math.round( frb.maxUSD * minAmount );
        // Math.round to account for floating point math errors: https://phabricator.wikimedia.org/T246262

    $('.frb-error-smallamount').text( function( index, oldText ) {
        return oldText.replace( '$1', frb.formatCurrency(currency, minAmount, language)  );
    });

    $('.frb-error-bigamount').text( function( index, oldText ) {
        // We cannot accept donations greater than $1 $2 through our website. Please contact our major gifts staff at $3.
        return oldText.replace( '$1', maxAmount )
                      .replace( '$2', currency )
                      .replace( '$3', 'benefactors@wikimedia.org' );
    });
};

/**
 * Shared code for amount input handling
 */
frb.initAmountOptions = function() {

    // Reset &quot;Other&quot; input if user clicks a preset amount
    $('#frb-form [id^=frb-amt-ps]').click(function() {
        $('#frb-amt-other-input').val('');
    });

    // Track if they selected and then later changed amount
    var checkAmountChange = function(e) {
        if ( frb.didSelectAmount ) {
            frb.extraData.changedAmt = 1;
        }
        // check if amount radio button is selected OR there is a value in the other amount
        if ( $('.frb-amounts input[type=&quot;radio&quot;]:checked').val() !== 'Other' || $('#frb-amt-other-input').val().length > 0 ) {
            frb.didSelectAmount = true;
        }
        return;
    };

    $('.frb-amounts input[type=&quot;radio&quot;]').on('change', checkAmountChange);
    $('#frb-amt-other-input').on('focusout', checkAmountChange);

    // Block typing non-numerics in input field, otherwise Safari allows them and then chokes
    // https://phabricator.wikimedia.org/T118741, https://phabricator.wikimedia.org/T173431
    var blockNonNumeric = function(e) {
        // Allow special keys in Firefox
        if ((e.code == 'ArrowLeft') || (e.code == 'ArrowRight') ||
            (e.code == 'ArrowUp') || (e.code == 'ArrowDown') ||
            (e.code == 'Delete') || (e.code == 'Backspace')) {
            return;
        }
        var chr = String.fromCharCode(e.which);
        if (&quot;0123456789., &quot;.indexOf(chr) === -1) {
            return false;
        }
    };
    $('#frb-amt-other-input').on('keypress', blockNonNumeric);
    $('#frb-amt-monthly-other-input').on('keypress', blockNonNumeric);

};

/**
 * Calculate approximate transaction fee on given amount
 *
 * @param  {number} amount
 * @return {number}        Rounded to 2 decimal places
 */
frb.calculateFee = function(amount) {
    var currency = frb.getCurrency(mw.centralNotice.data.country),
        feeMultiplier = 0.04,
        feeMinimum = frb.amounts.feeMinimums[currency] || 0.35,
        feeAmount = amount * feeMultiplier;

    if ( feeAmount &lt; feeMinimum ) {
      feeAmount = feeMinimum;
    }
    return parseFloat(feeAmount.toFixed(2));
};

frb.updateFeeDisplay = function() {
    var currency = frb.getCurrency(mw.centralNotice.data.country),
        language = mw.centralNotice.data.uselang,
        amount, feeAmount, totalAmount;

    amount = frb.getAmount();
    feeAmount = frb.calculateFee(amount);
    if ( $('#frb-ptf-checkbox').prop('checked') ) {
        totalAmount = amount + feeAmount;
    } else {
        totalAmount = amount;
    }

    var feeAmountFormatted = frb.formatCurrency(currency, feeAmount, language);
    $('.frb-ptf-fee').text(feeAmountFormatted);

    var totalAmountFormatted = frb.formatCurrency(currency, totalAmount, language);
    $('.frb-ptf-total').text(totalAmountFormatted);

    $('.frb-ptf').slideDown( frb.reduceMotion ? 0 : 400 );
};

/**
 * Custom hide cookie function
 *
 * Purposely sets only for this domain.
 * CentralNotice builtin method seems buggy - see T270401
 *
 * @param {string} reason Reason to store in the hide cookie
 * @param {number} duration Cookie duration, in seconds
 */
frb.altSetHideCookie = function ( reason, duration ) {

    mw.loader.using( 'mediawiki.cookie' ).then( function () {

        var cookieName = 'centralnotice_hide_fundraising',
            date = new Date(),
            hideData = {
                v: 1,
                created: Math.floor( date.getTime() / 1000 ),
                reason: reason
            };

        // Re-use the same date object to set the cookie's expiry time
        date.setSeconds( date.getSeconds() + duration );

        mw.cookie.set(
            cookieName,
            JSON.stringify( hideData ),
            { expires: date, path: '/', domain: 'wikipedia.org', prefix: '' }
        );

    });

};

/**
 * Determine if banner should be shown, and set correct data for impression logging
 *
 * @return {boolean} Show banner?
 */
frb.shouldShowBanner = function() {

    mw.centralNotice.bannerData.hideResult = false;

    /* Hide in unsupported browsers */
    if ( !frb.supportedBrowser ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = 'browser';
    }

    /* Hide outside main namespace (except Main Page, for sites where it isn't in main namespace) */
    if ( mw.config.get('wgNamespaceNumber') > 0 &amp;&amp; !mw.config.get('wgIsMainPage') ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = 'namespace';
    }

    // Hide banner on sensitive articles
    // TODO - possibly add wgWikibaseItemId for multilingual support and resilience to moves?
    var hideTitles = [ 'Murder of Don Banfield' ];
    if ( hideTitles.indexOf( mw.config.values.wgTitle ) !== -1 ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = 'article';
    }

    /* Hide banner if on wrong site (desktop/mobile) in case wrong device settings were chosen */
    var bannerName = mw.centralNotice.data.banner,
        skin = mw.config.get('skin');
    if (
         ( bannerName.indexOf('_dsk_') !== -1 &amp;&amp; skin === 'minerva' ) ||
         ( bannerName.indexOf('_m_') !== -1 &amp;&amp; skin !== 'minerva' )
    ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = 'other';
        console.warn('Hiding fundraising banner on wrong site (desktop/mobile)');
    }

    return !mw.centralNotice.bannerData.hideResult;

};

/* Debug function to highlight dynamically replaced elements */
frb.highlightReplacements = function() {
    $('.frb [class^=&quot;frb-replace&quot;], .frb-ptf-fee, .frb-ptf-total, .frb-upsell-ask, frb-amt').css('background-color', '#fa0');
};

if ( !mw.centralNotice.adminUi ) { // T262693
    /**
     * Provides alterImpressionData hook for CentralNotice
     * This info will be sent back with Special:RecordImpression
     * TODO: check if/when we can remove this (and RecordImpression)
     */
    mediaWiki.centralNotice.bannerData.alterImpressionData = function( impressionData ) {
        // Returning true from this function indicates the banner was shown
        if (mediaWiki.centralNotice.bannerData.hideReason) {
            impressionData.reason = mediaWiki.centralNotice.bannerData.hideReason;
        }
        if (mediaWiki.centralNotice.bannerData.cookieCount) {
            impressionData.banner_count = mediaWiki.centralNotice.bannerData.cookieCount;
        }

        return !mediaWiki.centralNotice.bannerData.hideResult;
    };
}

/* End of MediaWiki:FundraisingBanners/CoreJS-2018.js */
/* jshint maxerr: 600 */
/* == MediaWiki:FundraisingBanners/LocalizeJS-2022.js == */

/**
 * Get the currency for a given country
 *
 * NOTE: The following currency mapping is WMF-specific based on payment
 * provider availability, NOT necessarily the official currency of the country
 *
 * @param  {string} country code
 * @return {string} currency code
 */
frb.getCurrency = function(country) {
    switch ( country ) {
        // Big 6
        case 'US': return 'USD';
        case 'CA': return 'CAD';
        case 'AU': return 'AUD';
        case 'NZ': return 'NZD';
        case 'GB': return 'GBP';
        case 'IE': return 'EUR';
        // Euro countries
        case 'AT':
        case 'BE':
        case 'ES':
        case 'FR':
        case 'GR':
        case 'IE':
        case 'IT':
        case 'LU':
        case 'LV':
        case 'NL':
        case 'PT':
        case 'SK':
            return 'EUR';
        // Others
        case 'DK': return 'DKK';
        case 'HU': return 'HUF';
        case 'IL': return 'ILS';
        case 'IN': return 'INR';
        case 'JP': return 'JPY';
        case 'MY': return 'MYR';
        case 'NO': return 'NOK';
        case 'PL': return 'PLN';
        case 'CZ': return 'CZK';
        case 'RO': return 'RON';
        case 'SE': return 'SEK';
        case 'UA': return 'UAH';
        case 'ZA': return 'ZAR';
        // Latin America
        case 'BR': return 'BRL';
        case 'AR': return 'ARS';
        case 'CL': return 'CLP';
        case 'CO': return 'COP';
        case 'MX': return 'MXN';
        case 'PE': return 'PEN';
        case 'UY': return 'UYU';
        case 'CH': return 'CHF';
        case 'LI': return 'CHF';
        // Fall back to USD
        default:
            return 'USD';
    }
};

/**
 * Format a currency value
 *
 * TODO: make this handle the ISO code overrides?
 *
 * @param  {string} currency code. Leave undefined to get without symbol.
 * @param  {number} amount
 * @param  {string} language code
 * @return {string} formatted string e.g. '$3', '£5', '10 €'
 */
frb.formatCurrency = function(currency, amount, language) {

    var locale, formatterOptions, formatter, fmAmount, supportsIntl;

    if ( isNaN(amount) || amount === '' ) {
        // Not a number, it's probably the 'other' string or box
        // TODO: better way of doing this?
        fmAmount = amount;
    } else {
        // Check browser support
        try {
            supportsIntl = typeof window.Intl === 'object';
        } catch (e) {
            supportsIntl = false; // T265396
        }

        if ( supportsIntl ) {
            // Use Intl for fancy number formatting - thousands separators etc
            locale = language + '-' + mw.centralNotice.data.country;
            if ( amount % 1 !== 0 ) {
                formatterOptions = { minimumFractionDigits: 2 };
            } else {
                formatterOptions = {};
            }
            formatter = new Intl.NumberFormat(locale, formatterOptions);
        } else {
            // Bad browser i.e. IE. Just do the basics: 2 decimal places if needed, or none
            formatter = {};
            formatter.format = function(number) {
                if ( amount % 1 !== 0 ) {
                    return number.toFixed(2);
                } else {
                    return number.toString();
                }
            };
        }
        fmAmount = formatter.format(amount);
    }

    // No symbol needed
    if ( currency === undefined ) {
        return fmAmount;
    }

    // Better dive into the formatting object
    if ( frb.currencyFormats[currency] === undefined ) {
        return currency + '&amp;nbsp;' + fmAmount;
    }
    if ( frb.currencyFormats[currency] instanceof Object ) { // not a string
        if ( frb.currencyFormats[currency][language] !== undefined ) {
            return frb.currencyFormats[currency][language].replace('\t', fmAmount);
        }
        return frb.currencyFormats[currency]['default'].replace('\t', fmAmount);
    }

    return frb.currencyFormats[currency].replace('\t', fmAmount);
};

/*
 * Select the correct amount or array of amounts from object in &quot;source&quot;
 *
 * @param {Object} source   - the amounts data object e.g. frb.amounts.options7, frb.amounts.averages
 * @param {string} currency - ISO code of currency
 * @param {string} country  - ISO code of country (optional)
 * @return {array/number}   - depending on source
 */
frb.pickAmounts = function(source, currency, country) {

    if ( source[currency]['default'] ) { // we need to go deeper
        if ( source[currency][country] !== undefined ) {
            return source[currency][country];
        } else {
            return source[currency]['default'];
        }
    } else {
        return source[currency];
    }
};

/* Credit card types so we can show the correct logos */
frb.cardTypes = {
    // Big 6
    'US' : 'vmad',
    'CA' : 'vma',
    'GB' : 'vmaj',
    'IE' : 'vmaj',
    'AU' : 'vmaj',
    'NZ' : 'vma',
    // Euro countries
    'AT' : 'vmaj',
    'BE' : 'vmaj',
    'ES' : 'vmaj',
    'FR' : 'vma', // Adyen
    'GR' : 'vmaj',
    'IT' : 'vmaj',
    'LU' : 'vmaj',
    'LV' : 'vma',
    'NL' : 'vmaj', // Adyen
    'PT' : 'vmaj',
    'SK' : 'vmaj',
    // Others
    'CZ' : 'vmad',
    'DK' : 'vma',
    'HU' : 'vma',
    'IL' : 'vmad', // Adyen
    'JP' : 'vmaj',
    'MY' : 'vmaj',
    'NO' : 'vma',
    'PL' : 'vma',
    'RO' : 'vma',
    'SE' : 'vma',
    'UA' : 'vma', // Adyen
    'ZA' : 'vm',
    'IN' : 'vmar', // dLocal
    'CH' : 'vma',
    'LI' : 'vma'
};

/**
 * Should we show Apple Pay?
 *
 * Note there is a ~500ms delay in Safari when checking, so only call this if needed
 *
 * @param  {string} country
 * @return {boolean}
 */
frb.shouldShowApplePay = function ( country ) {
    // https://support.apple.com/en-us/HT207957 - minus China mainland
    var appleCountries = [
        'ZA',
        'AU', 'HK', 'JP', 'MO', 'NZ', 'SG', 'TW',
        'AM', 'AT', 'AZ', 'BY', 'BE', 'BG', 'HR', 'CY', 'CZ', 'DK', 
        'EE', 'FO', 'FI', 'FR', 'GE', 'DE', 'GR', 'GL', 'GG', 'HU', 
        'IS', 'IE', 'IM', 'IT', 'KZ', 'JE', 'LV', 'LI', 'LT', 'LU', 
        'MT', 'MC', 'ME', 'NL', 'NO', 'PL', 'PT', 'RO', 'RU', 'SM', 
        'RS', 'SK', 'SI', 'ES', 'SE', 'CH', 'UA', 'GB', 'VA', 
        'CO', 'CR', 'BR', 'MX', 
        'BH', 'IL', 'PS', 'QA', 'SA', 'AE', 
        'CA', 'US'
    ];
    if ( appleCountries.indexOf( country ) === -1 ) {
        return false;
    }
    if ( location.search.match('forceApplePay') ) {
        return true;
    }
    if ( window.ApplePaySession ) {
        if ( ApplePaySession.canMakePayments() ) {
            return true;
        }
    }
    return false;
};

/**
 * Display the correct payment methods for current country
 *
 * Methods should be labeled with class 'frb-pm-xxxx'
 * TODO: clean this function up more
 *
 * @param  {string} country
 */
frb.localizeMethods = function(country) {

    // Test country with *all the methods*
    if ( country === 'ZZ' ) {
        $('.frb-payment-options > div').show();
        return;
    }

    // Hide recurring completely for some countries and endowment
    if ( frb.isEndowment || frb.noRecurringCountries.indexOf(country) !== -1 ) {
        $('.frb-frequency, .recurring-details').hide();
    }

    // Remove any leftover WorldPay and Adyen
    $('.frb-pm-cc-wp').remove();
    $('.frb-pm-cc-adyen').remove();

    // Monthly Adyen credit card is allowed now
    // if ( frb.ccAdyenCountries.indexOf( country ) !== -1 ) {
    //     $('.frb-pm-cc').addClass('no-monthly');
    // }

    // Countries with no PayPal option
    var noPP = ['IN', 'RU', 'SG', 'AE', 'QA', 'OM', 'BD', 'BO', 'PA',
                'PY', 'GT', 'JM', 'TT', 'DZ'];
    if ($.inArray(country, noPP) !== -1) {
        $('.frb-pm-pp').remove();
        $('.frb-pm-pp-usd').remove();
    }

    // Countries with no PayPal for mobile only - https://phabricator.wikimedia.org/T173001
    var noPPmobile = ['PH', 'ID', 'TH', 'KR', 'MY', 'VN'];
    var mobileRegex = /(_mob_|_ipd_|_m_)/;
    if ($.inArray(country, noPPmobile) !== -1) {
        if (mw.centralNotice.data.banner.search(mobileRegex) !== -1) {
            $('.frb-pm-pp').remove();
            $('.frb-pm-pp-usd').remove();
        }
    }

    // Countries where PayPal must be in USD
    var ppUSD = ['BG', 'HR', 'LT', 'MK', 'RO', 'UA', 'SA', 'CN', 'ID', 'KR',
                 'KZ', 'MY', 'VN', 'AR', 'CL', 'DO', 'CO', 'NI', 'UY', 'ZA',
                 'BH', 'LB', 'VE', 'TR', 'IS', 'BA', 'MV', 'BB', 'BM', 'BZ',
                 'CR', 'CW', 'SX', 'HN', 'KN', 'DM', 'AG', 'LC', 'GD', 'FJ',
                 'TN', 'BJ', 'BF', 'CI', 'GW', 'ML', 'NE', 'SN', 'TG', 'BR',
                 'PE'];
    if ($.inArray(country, ppUSD) !== -1) {
        $('.frb-pm-pp').remove();
        $('.frb-pm-pp-usd').show();
    } else {
        $('.frb-pm-pp').show();
        $('.frb-pm-pp-usd').remove();
    }

    // Show any extra local payment methods, or remove them if not needed
    var extrapaymentmethods = {
        'amazon'   : ['US'], // Note Amazon was removed from current best 2023-10-20
        'bpay'     : [],
        'ideal'    : ['NL'],
        'bt'       : ['BR', 'AR', 'CO', 'CL', 'PE', 'MX', 'IN', 'ZA', 'CZ'], // Bank Transfer (dLocal/Adyen)
        'cash'     : ['MX', 'AR', 'CO', 'CL', 'PE', 'UY'],  // 'Cash' methods (dLocal)
        'pix'      : ['BR'],
        'boleto'   : ['BR']
    };

    // Methods with different labels per country

    var language = mw.config.get('wgUserLanguage');
    var btTranslation = 'Bank Transfer';

    if (language === 'pt') {

        if (country === 'BR') {
            btTranslation = 'Transferência bancária';
        }

    } else if (language === 'es') {

        if (country === 'CL') {
            btTranslation = 'WebPay';
        } else if (country === 'CO') {
            btTranslation = 'PSE Pagos';
        } else {
            btTranslation = 'Transferencia bancaria';
        }

    }

    if (country === 'CZ') {
        if (language === 'en') {
            btTranslation = 'Online Banking';
        }
        if (language === 'cs') {
            btTranslation = 'Internetové Bankovnictví';
        }
    }

    $( '.frb-pm-bt button, .frb-pm-bt label, button.frb-pm-bt' ).text( btTranslation );

    for (var method in extrapaymentmethods) {
        var $methodbutton = $('.frb-pm-' + method);
        if ( $.inArray(country, extrapaymentmethods[method]) !== -1 &amp;&amp; !frb.isEndowment ) {
            $methodbutton.show();
        } else {
            $methodbutton.remove();
        }
    }

    // Google Pay - separated from extrapaymentmethods as we want to show on Endowment too
    var googlePayCountries = [
        'AE', 'AT', 'AR', 'AU', 'BE', 'BG', 'BR', 'CA', 'CH', 'CL', 'CO',
        'CZ', 'DE', 'DK', 'EE', 'ES', 'FR', 'GB', 'GR', 'HK', 'HR',
        'HU', 'IE', 'IL', 'IT', 'JP', 'LU', 'LV', 'MX', 'MY', 'NL',
        'NO', 'NZ', 'OM', 'PE', 'PL', 'PT', 'QA', 'RO', 'RU', 'SA', 'SE',
        'SG', 'SK', 'TH', 'TR', 'TW', 'UA', 'US', 'UY', 'VN', 'ZA'
    ];
    if ( $.inArray(country, googlePayCountries) !== -1 ) {
        $('.frb-pm-google').show();
    } else {
        $('.frb-pm-google').remove();
    }

    // Apple Pay
    if ( $('.frb-pm-applepay').length > 0 ) {
        if ( !frb.shouldShowApplePay( country ) ) {
            $('.frb-pm-applepay').remove();
        }
    }

	// Venmo
	var $venmo = $('.frb-pm-venmo');
	if ( country === 'US' &amp;&amp; $venmo.length > 0 ) {
		// From MediaWiki:FundraisingBanners/VenmoBrowserCheck.js
		if ( frb.isVenmoSupported() ) {
			$venmo.show();
		} else {
			$venmo.remove();
		}
	} else {
		$venmo.remove();
	}

    /* Add card types class to credit card button, so we can show correct logos */
    if ( frb.cardTypes[country] ) {
        $('.frb-pm-cc').addClass('frb-cctypes-' + frb.cardTypes[country] );
    }
};

/**
 * Check scheduled payment method outages and hide buttons if needed
 *
 * Data at https://meta.wikimedia.org/wiki/MediaWiki:FR2013/Resources/PaymentOutages.js
 * Methods should be labeled with class 'frb-pm-xxxx'
 *
 * @param  {string} country code
 */
frb.checkMethodOutages = function(country) {

    // TODO - can we load this a better way?
    /* This file can be used to schedule hiding of individual payment methods from banners
 * e.g. if they have scheduled downtime.
 *
 * Valid methods are:
 *	ideal, cc, pp, amazon, bpay, webmoney, cash, pp-usd
 * (most of the time it's 'ideal'...)
 * Can also limit outage to a specific country with country: &quot;XX&quot; (where XX is an ISO code)
 *
 * Note that in JavaScript dates the months (and only the months) start at 0.
 * Jan=0, Feb=1, Mar=2, Apr=3 etc. How hateful.
 *
 * Be sure to also update donatewiki if needed e.g. by commenting the method templates
 * found at https://donate.wikimedia.org/wiki/Template:2012FR/Form-section/Paymentmethods
 * 
 */
var outages = [
    {
        start:      new Date(Date.UTC(2016, 8, 18, 1)),
        end:        new Date(Date.UTC(2016, 8, 18, 7)),
        method:     &quot;ideal&quot;
    }
]; // jshint ignore:line
    var now = new Date();

    for (var i = outages.length - 1; i >= 0; i--) {
        if ( now > outages[i].start &amp;&amp; now &lt; outages[i].end ) {
            if (outages[i].country === undefined || outages[i].country == country) {
                $('.frb-pm-' + outages[i].method).hide();
            }
        }
    }
};

/**
 * Adjust the amount options and their labels
 *
 * Inputs should have id frb-amt-psX where X is the index number (starting from 1)
 *
 * @param  {Object}  source     - object with amounts e.g. frb.amounts.options7
 * @param  {string}  currency   - currency code e.g. 'USD'
 * @param  {string}  country    - country code  e.g. 'FR' Some currencies can have different options per country.
 * @param  {string}  language   - language code e.g. 'en' For symbol formatting
 * @param  {boolean} useSymbols - use currency symbols on labels or not? (3 vs $3)
 */
frb.localizeAmountOptions = function(source, currency, country, language, useSymbols) {

    var amountOptions = frb.pickAmounts(source, currency, country);

    $('#frb-form input[name=&quot;amount&quot;]').each(function(index) {
        var $input = $(this);
        var $label = $input.siblings('label');

        var i = $input.attr('id').replace('frb-amt-ps', '');
        var amount = amountOptions[i-1]; // because IDs start from 1

        if ( amount ) {
            $input.val( amount );
            if ( useSymbols ) {
                $label.text( frb.formatCurrency( currency, amount, language) );
            } else {
                $label.text( frb.formatCurrency( undefined, amount, language) );
            }
        }
    });

};

/**
 * Make an element into a link
 *
 * @param  {string} selector    CSS selector for elements to convert to a link
 * @param  {string} language    Code of language (could be es-419 or pt-br)
 * @param  {string} baseUrl     URL of link (function will add language parameter)
 */
frb.makeLink = function( selector, language, baseUrl ) {
    var url = baseUrl + '&amp;language=' + language;
    $( selector ).each( function() {
        var $link = $( '&lt;a>&lt;/a>' );
        $link.html( $( this ).html() );
        $link.attr( { href: url, target: '_blank' } );
        $( this ).replaceWith( $link );
    });
};

/**
 * Get the number of banners seen from localStorage
 * @return {number} Number of banners seen
 */
frb.getSeenCount = function () {

    // Force with URL parameter 'impression'
    if ( typeof URLSearchParams === 'function' ) { // not available in old browsers
        var urlParams = new URLSearchParams( window.location.search );
        if ( urlParams.has( 'impression' ) ) {
            return urlParams.get( 'impression' );
        }
    }

    try {
        if ( localStorage ) {
            var identifier = mw.centralNotice.internal.state.campaign.mixins.impressionDiet.cookieName,
                lsName = 'CentralNoticeKV|global|impression_diet_' + identifier,
                diet = JSON.parse( localStorage.getItem( lsName ) );
            if ( diet ) {
                return diet.val.seenCount;
            }
        }
    } catch ( ex ) {
        // do nothing - localStorage is configured not to let us read it, or mixin not set
        return;
    }
};

/**
 * Helper function to do text replacements and wrap them in correct class
 * 
 * @param  {RegExp} regex       Regular expression to replace
 * @param  {string} replacement String to replace it with
 */
frb.textReplace = function( regex, replacement ) {
    $( '.frb' ).each( function( index ) {
        var newHtml = $( this ).html();
        newHtml = newHtml.replace( regex, '&lt;span class=&quot;frb-replaced&quot;>' + replacement + '&lt;/span>' );
        $( this ).html( newHtml );
    });
};

/**
 * Replace elements with preset ask string amounts
 *
 * e.g. class=&quot;frb-replace-amt-ps4&quot; will be replaced with amount #4, currently $25 in the US
 *
 * @param  {string} currency - currency code e.g. 'USD'
 * @param  {string} country  - country code  e.g. 'FR'
 * @param  {string} language - language code e.g. 'en' For symbol formatting
 */
frb.replaceCustomAmounts = function( currency, country, language ) {
    var amountOptions = frb.pickAmounts( frb.amounts.options7, currency, country );

    // Old style element replacements
    $( '.frb [class^=&quot;frb-replace-amt-ps&quot;]' ).each( function() {
        var i = $( this ).attr( 'class' ).replace( 'frb-replace-amt-ps', '' ),
            amount = amountOptions[ i - 1 ],
            formattedAmount = frb.formatCurrency( currency, amount, language );
        $( this ).html( '&lt;frb-amt>' + formattedAmount + '&lt;/frb-amt>' );
    });

    // Text replacements e.g. %amount-4%
    // There is probably a more efficient way to do this, but it's at least fairly simple
    for (var i = 0; i &lt; amountOptions.length; i++) {
        var amount = amountOptions[i],
            formattedAmount,
            regex = new RegExp( '%amount-' + (i+1) + '%', 'gi' );
        if ( frb.textAmountIsoCountries.includes( country ) ) {
            formattedAmount = frb.formatCurrency( undefined, amount, language ) + '&amp;nbsp;' + currency;
        } else {
            formattedAmount = frb.formatCurrency( currency, amount, language );
        }
        frb.textReplace( regex, formattedAmount );
    }
};

/**
 * Get today's date like &quot;December 3&quot; - English only for now
 * 
 * @return {string} Today's date as a string
 */
frb.getDateString = function() {
    var date = new Date(),
        locale = mw.centralNotice.data.uselang + '-' + mw.centralNotice.data.country;
    return date.toLocaleString( locale, { day: 'numeric', month: 'long' } );
};

frb.noRecurringCountries = ['AR'];
frb.ccAdyenCountries     = ['FR', 'IL', 'UA'];

/* These countries use potentially ambiguous $ sign.
Use ISO code instead in text (but still $ for buttons) */
frb.textAmountIsoCountries = ['AR', 'CL', 'CO', 'MX'];

$(function() {

    if ( mw.centralNotice.adminUi ) { // T262693
        return;
    }

    var language = mw.centralNotice.data.uselang;
    var variantLanguage; // for pt-br and es-419, note we can only use these for certain links
    var country  = mw.centralNotice.data.country;
    var currency = frb.getCurrency(country);

    if ( language === 'pt' &amp;&amp; country === 'BR' ) {
        variantLanguage = 'pt-br';
    } else if ( language === 'es' &amp;&amp; ['AR', 'CL', 'CO', 'PE', 'MX', 'UY'].indexOf( country ) !== -1 ) {
        variantLanguage = 'es-419';
    } else {
        variantLanguage = language;
    }

    // Payment methods
    frb.localizeMethods(country);
    frb.checkMethodOutages(country);

    // Preset amounts
    frb.replaceCustomAmounts( currency, country, language );

    // Basic replacements
    $('.frb-replace-currencysymbol').text( frb.formatCurrency( currency, '', language ).replace(' ', '') );
    $('.frb-replace-currencycode').text( currency );

    // Country name
    var countryName;
    if ( frb.countryNames[language] ) {
        countryName = frb.countryNames[language][country] || frb.countryNames.en[country];
    } else {
        countryName = frb.countryNames.en[country];
    }
    $( '.frb-replace-countryname' ).text( countryName );
    frb.textReplace( /%country%/gi, countryName );

    // &quot;in COUNTRY&quot; or equivalent
    var inCountryName;
    if ( frb.inCountryNames[language] ) {
        inCountryName = frb.inCountryNames[language][country] || frb.inCountryNames.en[country];
    } else {
        inCountryName = frb.inCountryNames.en[country];
    }
    $( '.frb-replace-incountryname' ).text( inCountryName );
    frb.textReplace( /%in-country%/gi, inCountryName );

    // Day of week
    // TODO: Replace these with date.toLocaleString so we can drop frb.dayNames? 
    //       Might still need some ways to deal with &quot;this&quot; and capitalization
    var now = new Date();
    var dayNumber = now.getDay();
    var capitalizeText = function( text ) {
        // Capitalize first letter, for use at start of sentence
        return text.charAt(0).toUpperCase() + text.slice(1);
    };

    if ( $('.frb-replace-dayofweek, .frb-replace-dayofweek-capitalize').length > 0 ) {
        if ( frb.dayNames[language] ) {
            $('.frb-replace-dayofweek').text( frb.dayNames[language][dayNumber] );
            $('.frb-replace-dayofweek-capitalize').text( capitalizeText( frb.dayNames[language][dayNumber] ) );
        } else {
            console.log('Warning: banner should contain a day of the week, but no translations found.');
        }
    }

    if ( $('.frb-replace-dayofweek-this, .frb-replace-dayofweek-this-capitalize').length > 0 ) {
        if ( frb.dayNamesThis[language] ) {
            $('.frb-replace-dayofweek-this').text( frb.dayNamesThis[language][dayNumber] );
            $('.frb-replace-dayofweek-this-capitalize').text( capitalizeText( frb.dayNamesThis[language][dayNumber] ) );
        } else {
            console.log('Warning: banner should contain &quot;this DAY&quot;, but no translations found.');
        }
    }

    // Simple %weekday% text replacement
    try {
        if ( frb.dayNames[language] ) {
            frb.textReplace( /%weekday%/gi, frb.dayNames[language][dayNumber] );
        } else {
            frb.textReplace( /%weekday%/gi, frb.dayNames['en'][dayNumber] );
        }
    } catch ( error ) {
        console.error( error );
    }

    // Replace %date% with today's date e.g. &quot;December 3&quot;
    try {
        frb.textReplace( /%date%/gi, frb.getDateString() );
    } catch ( error ) {
    	console.log( error );
    }

    // Capitalize
    $('.frb-capitalize').text(function( index, text ) {
        return text.charAt(0).toUpperCase() + text.slice(1);
    });

    // Replace %average%, %minimum% and %amount%
    var average = frb.pickAmounts( frb.amounts.averages, currency, country ),
        ifEveryone = frb.pickAmounts( frb.amounts.ifEveryone, currency, country ),
        avgString,
        ifString;

    if ( frb.textAmountIsoCountries.indexOf(country) !== -1 ) {
        avgString = frb.formatCurrency( undefined, average, language ) + '&amp;nbsp;' + currency;
        ifString  = frb.formatCurrency( undefined, ifEveryone, language ) + '&amp;nbsp;' + currency;
    } else {
        avgString = frb.formatCurrency( currency, average, language ).replace( /\.$/, '' ); // strip any period from end for use in running text
        ifString  = frb.formatCurrency( currency, ifEveryone, language ).replace( /\.$/, '' );
    }
    frb.textReplace( /%average%/gi, avgString );
    frb.textReplace( /%minimum%/gi, ifString );
    frb.textReplace( /%amount%/gi,  ifString );

    /**
     * Call a function on every text node contained by a root node.
     *
     * Used so we can do text replacements without accidentally clobbering html and scripts
     *
     * @param  {Node}     rootNode The Node object whose descendants will be recursed through
     * @param  {Function} callback Callback function that receives a Node as its only argument
     */
    function eachTextNode( rootNode, callback ) {
        for ( var node = rootNode.firstChild; node !== null; node = node.nextSibling ) {
            if ( node.nodeType === Node.TEXT_NODE ) {
                callback( node );
            } else if ( node.nodeType === Node.ELEMENT_NODE ) {
                eachTextNode( node, callback );
            }
        }
    }

    // French spacing: replace space before punctuation with &amp;nbsp;
    if ( language === 'fr' ) {
        var bannerRootElements = document.getElementsByClassName( 'frb' );
        for ( var i = 0; i &lt; bannerRootElements.length; i++ ) {
            eachTextNode( bannerRootElements[i], function( node ) {
                node.textContent = node.textContent.replace( / ([!?;:%])/g, '\u00a0$1' );
            });
        }
    }

    // Links (in smallprint) TODO: merge with frb.makeLink()
    $('.frb-localize-links a').each(function() {
        // Add parameters for LandingCheck
        var uri = new mw.Uri( $(this).attr('href') );
        uri.extend({
            country:      country,
            language:     variantLanguage,
            uselang:      variantLanguage,
            utm_medium:   'sitenotice',
            utm_campaign: mw.centralNotice.data.campaign || 'test',
            utm_source:   mw.centralNotice.data.banner
        });
        $(this).attr('href', uri.toString());
        $(this).attr('target', '_blank'); // Make links open in new tab
    });

    // Add links
    frb.makeLink( '.frb-link-privacy', variantLanguage, 'https://foundation.wikimedia.org/wiki/Special:LandingCheck?basic=true&amp;landing_page=Donor_privacy_policy' );
    frb.makeLink( '.frb-link-tax',     variantLanguage, 'https://donate.wikimedia.org/wiki/Special:LandingCheck?basic=true&amp;landing_page=Tax_deductibility' );
    frb.makeLink( '.frb-link-cancel',  variantLanguage, 'https://donate.wikimedia.org/wiki/Special:LandingCheck?basic=true&amp;landing_page=Cancel_or_change_recurring_giving' );

    // Legal text variants
    if (country === 'US') {
        $('.frb-legal-US').show();
        $('.frb-legal-nonUS, .frb-legal-NL').hide();
    } else if (country === 'NL') {
        $('.frb-legal-NL').show();
        $('.frb-legal-US, .frb-legal-nonUS').hide();
    } else {
        $('.frb-legal-nonUS').show();
        $('.frb-legal-US, .frb-legal-NL').hide();
    }

    // Quick hack for American/British/Commonwealth English differences
    if ( country === 'US' ) {
        $('.frb-lang-enUS').show();
        $('.frb-lang-en5C').hide();
    } else {
        $('.frb-lang-enUS').hide();
        $('.frb-lang-en5C').show();
    }

    // Add this so they get white-space: nowrap from CSS
    $('.frb-ptf-fee, .frb-ptf-total, .frb-upsell-ask').addClass('frb-replaced');

    // Where Remind Me Later should be shown
    var rmlCountries = ['US', 'CA', 'GB', 'IE', 'AU', 'NZ',
                        'IN', 'FR', 'NL'];
    var rmlLanguages = ['en', 'nl', 'ja', 'it', 'sv', 'pt', 'cs'];
    var rmlEnabled = !frb.isEndowment &amp;&amp; rmlCountries.indexOf(country) !== -1 &amp;&amp; rmlLanguages.indexOf(language) !== -1;

    if ( rmlEnabled ) {
        $('.frb').addClass('frb-rml-enabled');
    } else {
        $('.frb').addClass('frb-rml-disabled');
    }

});

/* == end of MediaWiki:FundraisingBanners/LocalizeJS-2022.js == */
/* eslint-env es6 */
var frb = frb || {};

/* Based on github:braintree/braintree-web/src/venmo/shared/supports-venmo.js */
frb.isVenmoSupported = function(options) {
  var options = options || {
    allowNewBrowserTab: false,
    allowWebviews: true,
    allowDesktop: true,
    allowDesktopWebLogin: true
  };
  var ua = window.navigator.userAgent;

  var merchantAllowsReturningToNewBrowserTab,
    merchantAllowsWebviews,
    merchantAllowsDesktopBrowsers;
  var isMobileDevice = isAndroid() || isIos();
  var isAndroidChrome = isAndroid() &amp;&amp; isChrome();
  var isMobileDeviceThatSupportsReturnToSameTab = isIosSafari() || isAndroidChrome;
  var isKnownUnsupportedMobileBrowser = isIosChrome() || isFacebookOwnedBrowserOnAndroid() || isSamsung();

  options = options || {};
  // NEXT_MAJOR_VERSION allowDesktop will default to true, but can be opted out
  merchantAllowsDesktopBrowsers =
    (options.allowDesktopWebLogin || options.allowDesktop) === true;
  merchantAllowsReturningToNewBrowserTab = options.hasOwnProperty(
    &quot;allowNewBrowserTab&quot;
  )
    ? options.allowNewBrowserTab
    : true;
  // NEXT_MAJOR_VERSION webviews are not supported, except for the case where
  // the merchant themselves is presenting venmo in a webview using the deep
  // link url to get back to their app. For the next major version, we should
  // just not have this option and instead require the merchant to determine
  // if the venmo button should be displayed when presenting it in the
  // merchant's app via a webview.
  merchantAllowsWebviews = options.hasOwnProperty(&quot;allowWebviews&quot;)
    ? options.allowWebviews
    : true;

  if (isKnownUnsupportedMobileBrowser) {
    return false;
  }

  if (
    !merchantAllowsWebviews &amp;&amp;
    (isAndroidWebview() || isIosWebview())
  ) {
    return false;
  }

  if (!isMobileDevice) {
    return merchantAllowsDesktopBrowsers;
  }

  if (!merchantAllowsReturningToNewBrowserTab) {
    return isMobileDeviceThatSupportsReturnToSameTab;
  }

  return isMobileDevice;

  /* -- functions mostly from github:braintree/browser-detection library -- */

  function isAndroid() {
    return /Android/i.test(ua);
  }

  function isIos(checkIpadOS = true) {
    const iOsTest = /iPhone|iPod|iPad/i.test(ua);
    return checkIpadOS ? iOsTest || isIpadOS() : iOsTest;
  }

  function isIpadOS() {
    // &quot;ontouchend&quot; is used to determine if a browser is on an iPad, otherwise
    // user-agents for iPadOS behave/identify as a desktop browser
    return /Mac|iPad/i.test(ua) &amp;&amp; &quot;ontouchend&quot; in window.document;
  }

  function isEdge() {
    return ua.indexOf(&quot;Edge/&quot;) !== -1 || ua.indexOf(&quot;Edg/&quot;) !== -1;
  }

  function isSamsung() {
    return /SamsungBrowser/i.test(ua);
  }

  function isDuckDuckGo() {
    return ua.indexOf(&quot;DuckDuckGo/&quot;) !== -1;
  }

  function isOpera() {
    return (
      ua.indexOf(&quot;OPR/&quot;) !== -1 ||
      ua.indexOf(&quot;Opera/&quot;) !== -1 ||
      ua.indexOf(&quot;OPT/&quot;) !== -1
    );
  }

  function isSilk() {
    return ua.indexOf(&quot;Silk/&quot;) !== -1;
  }

  function isChrome() {
    return (
      (ua.indexOf(&quot;Chrome&quot;) !== -1 || ua.indexOf(&quot;CriOS&quot;) !== -1) &amp;&amp;
      !isEdge() &amp;&amp;
      !isSamsung() &amp;&amp;
      !isDuckDuckGo() &amp;&amp;
      !isOpera() &amp;&amp;
      !isSilk()
    );
  }

  function isIosFirefox() {
    return /FxiOS/i.test(ua);
  }

  function isWebkit() {
    const webkitRegexp = /webkit/i;
    return webkitRegexp.test(ua);
  }

  function isIosChrome() {
    return ua.indexOf(&quot;CriOS&quot;) > -1;
  }

  function isFacebook() {
    return ua.indexOf(&quot;FBAN&quot;) > -1;
  }

  function isIosSafari() {
    return (
      isIos() &amp;&amp;
      isWebkit() &amp;&amp;
      !isIosChrome() &amp;&amp;
      !isIosFirefox() &amp;&amp;
      !isFacebook()
    );
  }

  function isFacebookOwnedBrowserOnAndroid() {
    var e = ua.toLowerCase();
    return -1 &lt; e.indexOf(&quot;huawei&quot;) &amp;&amp; -1 &lt; e.indexOf(&quot;fban&quot;) || isAndroid() &amp;&amp; (-1 &lt; e.indexOf(&quot;fb_iab&quot;) || -1 &lt; e.indexOf(&quot;instagram&quot;));
  }

  function isSamsungBrowser() {
    return /SamsungBrowser/i.test(ua);
  }

  function isAndroidWebview() {
    return isAndroid() &amp;&amp; -1 &lt; ua.toLowerCase().indexOf(&quot;wv&quot;);
  }

  function isGoogleSearchApp() {
    return /\bGSA\b/.test(ua);
  }

  function isIosGoogleSearchApp() {
    return isIos() &amp;&amp; isGoogleSearchApp();
  }

  function isIosWebview() {
    if (isIos()) {
      // The Google Search iOS app is technically a webview and doesn't support popups.
      if (isIosGoogleSearchApp()) {
        return true;
      }
      // Historically, a webview could be identified by the presence of AppleWebKit and _no_ presence of Safari after.
      return /.+AppleWebKit(?!.*Safari)/i.test(ua);
    }
    return false;
  }
};

$(function() {

    if ( mw.centralNotice.adminUi || !frb.supportedBrowser ) { // T262693
        return;
    }

    var language = mw.centralNotice.data.uselang;
    var country  = mw.centralNotice.data.country;
    var currency = frb.getCurrency(country);
    var validAmount;
    var validMethod;
    var validOptin;
    var form = document.getElementById('frb-form');

    var animationDuration = frb.reduceMotion ? 0 : 400;

    mw.loader.using(['mediawiki.util']).then(function() {
        frb.rml.init();
    });

    frb.initAmountOptions();
    frb.localizeAmountOptions( frb.amounts.options7, currency, country, language, true );
    frb.localizeErrors();

    frb.storedOptions = {};
    frb.extraData = {};

    frb.setMethod = function (options, frequency) {
        frb.storedOptions = options;

        if( frequency === 'no-monthly' ) {
            $('#frb-frequency-monthly').attr('disabled', true);
        } else {
            $('#frb-frequency-monthly').attr('disabled', false);
        }
    };

    frb.updateUpsellAsk = function(isOtherAmountStep) {
        var amount, feeAmount, upsellAmount,
            list = frb.amounts.monthlySuggest[currency];

        if ( list === undefined ) {
            console.log('No monthlySuggest amounts found for ' + currency);
            return;
        }

        // If user is on third step (write a different amount) then get monthly amount if not, the the first form amount
        if (isOtherAmountStep !== undefined) {
            amount = frb.getMonthlyAmount();
        } else {
            amount = frb.getAmount(form);
        }

        // If PTF is checked when we need to calculate the fee for that amount
        if ( $('#frb-ptf-checkbox').prop('checked') ) {
            amount = amount + frb.calculateFee(amount);
        }

        for (var i = list.length - 1; i >= 0; i--) {
            if ( amount &lt;= list[i][0] ) {
                upsellAmount = list[i][1];
            }
        }

        // If user is in the upsell (second step) then the form.otherMonthlyAmount.value will be updated with the upsellAmount calculated
        if (isOtherAmountStep === undefined) {
            form.otherMonthlyAmount.value = upsellAmount;
        }

        // A formatted value will be returned
        var upsellAmountFormatted = frb.formatCurrency(currency, upsellAmount, language);

        // The value of the amount will be updated only if the user is in the upsell (second step)
        if (isOtherAmountStep === undefined) {
            $('.frb-upsell-ask').text(upsellAmountFormatted);
        }
    };

    $('.frb-amounts input:not(#input_amount_other)').on('input change', function(e) {

        // Deal with https://phabricator.wikimedia.org/T191417
        if ( this.value === &quot;&quot; ) {
            return;
        }

        if ( frb.validateAmount() ) {
            validAmount = 1;
        } else {
            validAmount = 0;
        }
        frb.updateFeeDisplay();
        frb.activateCTA();
    });

    $('.frb-methods').on('change', function() {
        $('.frb-methods').removeClass('frb-haserror');
        $('.frb-error-method').hide();
        $('.frb-optin').slideDown( animationDuration );
        validMethod = 1;
        frb.activateCTA();
    });

    // Opt-in interaction
    $('.frb-optin').on('change', function() {
        $('.frb-optin').removeClass('frb-haserror');
        $('.frb-error-optin').hide();
        if ( $('#frb-optin-no').is(':checked') ) {
            $('.frb-optin-no-prompt').removeClass('is-positive');
            $('.frb-optin-no-prompt').slideDown( animationDuration );
        } else {
            $('.frb-optin-no-prompt').addClass('is-positive');
        }
        validOptin = 1;
        frb.activateCTA();
    });

    // Go to the next step of the form
    $('#frb-continue').on('click', function(e) {
        e.preventDefault();
        var status = {amount: false, method: false};

        // Validate amount
        if( frb.validateAmount() ){
            status.amount = true;
        } else {
            frb.extraData.validateError = 1;
        }

        // Validate method
        if ($('input[name=&quot;frb-methods&quot;]:checked').length === 1) {
            status.method = true;
        } else {
            frb.extraData.validateError = 1;
            $('.frb-methods').addClass('frb-haserror');
            $('.frb-error-method').show();
        }

        if (status.amount === true &amp;&amp; status.method === true) {

            frb.updateUpsellAsk();

            $('.frb-rml-link, .frb-rml').hide();

            if ( frb.optinRequired ) {
                frb.showStep('optin');
                setTimeout( () => {
                    $('.frb-rml-link').css({ 'visibility': 'hidden' });
                }, 300 );
            } else if ( frb.shouldShowMonthlyConvert() ) {
                frb.showStep('upsell');
                setTimeout( () => {
                    $('.frb-rml-link').css({ 'visibility': 'hidden' });
                }, 300 );
            } else {
                frb.submitForm( frb.storedOptions );
            }
        }
    });

    /* -- Back buttons -- */
    $('.frb-step-optin .frb-back').on('click', function(e) {
        frb.showStep('1');
        return false;
    });

    $('.frb-step-upsell .frb-back').on('click', function(e) {
        if ( frb.optinRequired ) {
            frb.showStep('optin');
        } else {
            frb.showStep('1');
        }
        return false;
    });

    $('.frb-step-monthly-diff-amt .frb-back').on('click', function(e) {
        form.otherMonthlyAmount.value = '';
        frb.updateUpsellAsk();
        validAmount = 1;
        frb.activateCTA();
        frb.toggleMonthly(false);
        frb.showStep('upsell');
        return false;
    });

    // Donate monthly other amount
    $('.frb-monthly-diff-amt-link').on('click', function(e) {
        form.otherMonthlyAmount.value = '';
        validAmount = 0;
        frb.activateCTA();
        frb.toggleMonthly(true);
        frb.showStep('monthly-diff-amt');
        return false;
    });

    // Validate monthly other amount
    $('#frb-amt-monthly-other-input').on('input change', function(e) {
        if ( frb.validateMonthlyAmount() ) {
            validAmount = 1;
            frb.updateUpsellAsk(true);
        } else {
            validAmount = 0;
        }
        frb.activateCTA();
    });

    frb.getMonthlyAmount = function() {
        var amount = null;

        // Check the &quot;monthly other&quot; amount box
        if (form.otherMonthlyAmount.value !== '') {
            var otherMonthlyAmount = form.otherMonthlyAmount.value;
            otherMonthlyAmount = otherMonthlyAmount.replace(/[,.](\d)$/, ':$10');
            otherMonthlyAmount = otherMonthlyAmount.replace(/[,.](\d)(\d)$/, ':$1$2');
            otherMonthlyAmount = otherMonthlyAmount.replace(/[$£€¥,.]/g, '');
            otherMonthlyAmount = otherMonthlyAmount.replace(/:/, '.');
            amount = otherMonthlyAmount;
        }

        amount = parseFloat(amount);

        if ( isNaN(amount) ) {
            return 0;
        } else {
            var totalMonthlyAmountFormatted = frb.formatCurrency(currency, amount, language);
            $('.frb-monthly-total').text(totalMonthlyAmountFormatted);

            return amount;
        }
    };

    frb.validateMonthlyAmount = function() {
        var amount = frb.getMonthlyAmount();
        var currency  = frb.getCurrency( mw.centralNotice.data.country );
        var minAmount = frb.amounts.minimums[ currency ];

        if ( amount === null || isNaN(amount) || amount &lt;= 0 || amount &lt; minAmount ) {
            $('.frb-error-bigamount').hide();
            $('.frb-error-smallamount').show();
            return false;
        } else if ( amount > frb.maxUSD * minAmount ) {
            $('.frb-error-bigamount').show();
            return false;
        } else {
            $('.frb-error-smallamount, .frb-error-bigamount').hide();
            return true;
        }
    };

    frb.submitMonthly = function() {
        frb.extraData.monthlyUpsell = 1;
        frb.extraData.originalAmt = frb.getAmount().toString();

        frb.toggleMonthly(true);
        document.getElementById('input_amount_other').checked = true;
        document.getElementById('frb-ptf-checkbox').checked = false;
        form.otherAmount.value = form.otherMonthlyAmount.value;
        frb.submitForm(frb.storedOptions);
    };

    // Submit form
    $('#frb-monthly-donate-yes').on('click', function(e) {
        frb.submitMonthly();
        return false;
    });

    $('#frb-monthly-donate-no').on('click', function(e) {
        frb.submitForm(frb.storedOptions);
        return false;
    });

    $('#frb-donate-monthly-other').on('click', function (e) {
        if (frb.validateMonthlyAmount()) {
            frb.submitMonthly();
        }
        return false;
    });

    /**
     * Should we show pre-payment monthly convert?
     *
     * Only if: initial selection is one-time, suggested amount is not 0 (meaning skip),
     * payment method supports monthly, and payment method does not have post-payments monthly convert
     *
     * @returns boolean
     */
     frb.shouldShowMonthlyConvert = function() {
        let postPaymentMethods = [ 'cc', 'apple', 'google' ];
        if (
            frb.getRecurring( document.getElementById( 'frb-form' ) ) ||
            !frb.shouldShowRecurring( frb.storedOptions, mw.centralNotice.data.country ) ||
            form.otherMonthlyAmount.value == 0 ||
            postPaymentMethods.includes( frb.storedOptions.method )
        ) {
            return false;
        } else {
            return true;
        }
    }

    $('#frb-donate').on('click', function(e) {
        if ( frb.validateForm( frb.storedOptions) ) {
            if ( frb.shouldShowMonthlyConvert() ) {
                frb.showStep('upsell');
            } else {
                frb.submitForm( frb.storedOptions );
            }
        } else {
            frb.extraData.validateError = 1;
        }
        return false;
    });

    // Focus for #input_amount_other
    $('.frb-amt-other').click(function() {
        document.getElementById('input_amount_other').checked = true;
        frb.updateFeeDisplay();
        $('#frb-amt-other-input').focus();
    });

    // Activate #input_amount_other radio when tabbing into #frb-amt-other-input
    $('#frb-amt-other-input').focus(function() {
        document.getElementById('input_amount_other').checked = true;
        frb.updateFeeDisplay();
    });

    frb.activateCTA = function(){
        if ( validAmount &amp;&amp; validMethod ) {
            $('.frb-submit-txt-continue').hide();
            $('.frb-submit-txt-donate').show();
            $('#frb-continue, #frb-monthly-donate-yes, #frb-monthly-donate-no, #frb-donate-monthly-other').addClass('active');
            if (validOptin) {
                $('#frb-donate').addClass('active');
            }
        } else {
            $('.frb-submit').removeClass('active');
        }
    };


    /* --- Nag/minimized banner functionality --- */

    // On Load
    var bannerOuterHeight = $('.frb-in-article').outerHeight( true );
    var stickyHeaderTop = bannerOuterHeight + $('.frb-in-article').offset().top + 200;

    frb.initNag = function() {

        // Intercept TOC clicks, and account for nag height
        $('#toc ul > li a').on('click.frb', function(e) {
            e.preventDefault();

            var anchor = $(this).attr('href').replace('#','');
            anchor = $(&quot;[id='&quot;+$.escapeSelector(anchor)+&quot;']&quot;);

            var offsetTop = anchor.offset().top - $('.frb-nag').outerHeight();
            $('body, html').animate({ scrollTop: offsetTop }, 10);
            window.location.hash = $(this).attr('href');
        });

        // Scroll to section, accounting for nag height
        if ( window.location.hash ) {
            var offsetTop;
            var hash = decodeURI(window.location.hash).replace('#','');
            hash = $(&quot;[id='&quot;+$.escapeSelector(hash)+&quot;']&quot;);
            if ( hash.offset() ) { // T281547
                offsetTop = hash.offset().top + bannerOuterHeight - $('.frb-nag').outerHeight();
                $('body, html').animate( { scrollTop: offsetTop }, 100 );
            }
        }

        $(window).on('resize.frb', function() {
            bannerOuterHeight = $('.frb-in-article').outerHeight( true );
            stickyHeaderTop = bannerOuterHeight + $('.frb-in-article').offset().top + 200;
        });

        function scrollFunction() {
            if( $(window).scrollTop() > stickyHeaderTop ) {
                if ( !frb.fixed ) {
                    $('.frb-nag').show();
                    $('.frb-rml-form-wrapper').hide();
                    $('.frb-rml').appendTo('.frb-nag .frb-form-wrapper');
                }
            } else {
                if ( frb.fixed ) {
                    $('.frb-prevent-page-jump')
                        .removeClass('frb-in-article')
                        .hide();
                    $('#frb-main')
                        .removeClass('frb-fixed')
                        .addClass('frb-in-article')
                        .trigger('unFixed');
                    $('.frb-rml').appendTo('#frb-main .frb-form-wrapper');
                    frb.fixed = false;
                } else {
                    $('.frb-nag').hide();
                    $('.frb-rml').appendTo('#frb-main .frb-form-wrapper');
                }
            }
        }

        $(window).on('load.frb scroll.frb resize.frb', function() {
            scrollFunction();
        });

        frb.clickNag = function(e) {
            frb.extraData.clickedNag = 1;

            if ( window.innerHeight &lt; document.getElementById('frb-main').offsetHeight ) {
                // Window height too short for fixing position, just jump to main banner
                window.scrollTo(0, 0);
                return false;
            }

            // Add spacer to prevent jump
            var inArticleHeight = $('#frb-main').outerHeight();
            $('.frb-prevent-page-jump')
                .height( inArticleHeight )
                .addClass('frb-in-article') // So that it can be used for stickyHeader calcs
                .show();

            $('#frb-main')
                .removeClass('frb-in-article')
                .addClass('frb-fixed');

            $('.frb-rml').appendTo('#frb-main .frb-form-wrapper');
            $('.frb-nag').hide();

            frb.fixed = true;
            return false;
        };

        $('.frb-nag').on( 'click', frb.clickNag );
        $('#nag-yes-btn').on( 'click', frb.clickNag );

        $('#nag-rml-btn').on( 'click', function(e) {
            // Add '_nag' to RML source value
            $('#frb-rml-form input[name=&quot;rml_source&quot;]').val('B2324_121810_en6C_dsk_p1_lg_txt_169C_nag');
            $('.frb-nag').addClass('frb-rml-displayed');
            $('.frb-nag').off('click'); // Remove so they can interact with RML
            $('#frb-rml-email').focus();
            $('.frb-rml-close-wrapper').hide();
        });

    };

    $('.back-rml').on('click', function(e) {
        $('.frb-nag').removeClass('frb-rml-displayed');
    });

    // Close inline rml form on click or return
    $('.frb-rml-close').on('click', function (e) {
        $('.frb-rml-form-wrapper').hide();
        e.stopPropagation();
    });

    $('.frb-close').on('click', function (e) {
        frb.hide();
        frb.altSetHideCookie( 'close', frb.HIDE_DURATION_CLOSE );
        frb.showSidebarTooltip();
        return false;
    });

    // Open already donated modal
    $('.modal-open').on('click', function(e) {
        $('.modal').show();
        $('button.modal-close-x').focus();
    });

    // Close already donated modal
    $('.modal-close').on('click', function(e) {
        closeModal(e);
        $('.modal-open').focus();
    });

    $('.modal').click(function(e) {
        const modalContainer = $('.modal-container');

        if (!modalContainer.is(e.target) &amp;&amp; modalContainer.has(e.target).length === 0) {
            closeModal(e);
            $('.modal-open').focus();
        }
    });

    function closeModal(e) {
        $('.modal').hide();
        if (e.target.name == 'frb-modal-close-button') {
            frb.hide();
            frb.altSetHideCookie( 'donate close', 604800 );
            e.target.blur();
        }
        return false;
    };

    $('#frb-main').on('unFixed', function() {
        $('.frb-rml').css( 'top', $('.frb-rml-link').position().top + 40 );
    });

    if ( country == &quot;US&quot; || country == &quot;CA&quot; || country == &quot;GB&quot; || country == &quot;IE&quot; || country == &quot;AU&quot; || country == &quot;NZ&quot; ) {
        $('#frb-main').addClass('frb-6c');
    }

    if ( frb.shouldShowBanner() ) {
        frb.initNag();
        frb.show();
    }

});

		</value>
      <webElementGuid>ff4072a0-5060-47eb-b177-19dd2bab5d99</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[@class=&quot;client-js vector-feature-language-in-header-enabled vector-feature-language-in-main-page-header-disabled vector-feature-sticky-header-disabled vector-feature-page-tools-pinned-disabled vector-feature-toc-pinned-clientpref-1 vector-feature-main-menu-pinned-disabled vector-feature-limited-width-clientpref-1 vector-feature-limited-width-content-enabled vector-feature-zebra-design-enabled vector-feature-custom-font-size-clientpref-0 vector-feature-client-preferences-disabled vector-feature-client-prefs-pinned-disabled vector-toc-not-available vector-animations-ready ve-available&quot;]/body[@class=&quot;skin-vector skin-vector-search-vue mediawiki ltr sitedir-ltr mw-hide-empty-elt ns-0 ns-subject page-Main_Page rootpage-Main_Page skin-vector-2022 action-view uls-dialog-sticky-hide&quot;]/div[@class=&quot;mw-page-container&quot;]/div[@class=&quot;mw-page-container-inner&quot;]/div[@class=&quot;vector-sitenotice-container&quot;]</value>
      <webElementGuid>e245547e-9730-45b1-bf9c-029c2624c149</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div/div</value>
      <webElementGuid>dd023336-4178-47c3-823c-57aa6992df2b</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
			

/* css variables */
:root {
    --wmui-base100: #fff;
    --wmui-base90: #f8f9fa;
    --wmui-base80: #eaecf0;
    --wmui-base70: #c8ccd1;
    --wmui-base50: #a2a9b1;
    --wmui-base30: #72777d;
    --wmui-base20: #54595d;
    --wmui-base10: #202122;
    --wmui-base0: #000;
    --wmui-accent: #36c;
    --wmui-accent-light: #eaf3ff;
    --wmui-accent-dark: #2a4b8d;
    --wmui-red: #d33;
    --wmui-red-light: #fee7e6;
    --wmui-red-dark: #b32424;
    --wmui-green: #00af89;
    --wmui-green-light: #d5fdf4;
    --wmui-green-dark: #14866d;
    --wmui-yellow: #fc3;
    --wmui-yellow-light: #fef6e7;
    --wmui-yellow-dark: #ac6600;
    --frb-primary: #2e5cb8;
    --frb-primary-light: #dde4f3;
    --frb-primary-dark: #2a4b8d;
    --frb-body: var(--wmui-base0);
    --frb-link: var(--wmui-accent);
    --frb-link-hover: #447ff5;
    --frb-message-background: var(--wmui-base100);
    --frb-message: var(--wmui-base0);
    --frb-message-border: #900;
    --frb-muted: var(--wmui-base20);
    --frb-muted-hover: var(--wmui-base0);
    --frb-radio: var(--wmui-accent);
    --frb-button: var(--wmui-base90);
    --frb-button-border: var(--wmui-base50);
    --frb-button-hover: var(--wmui-accent-light);
    --frb-button-border-hover: var(--wmui-base50);
    --frb-button-focus: var(--wmui-accent-light);
    --frb-button-border-focus: var(--wmui-base50);
    --frb-button-selected: var(--frb-primary-dark);
    --frb-button-border-selected: var(--frb-primary-dark);
    --frb-submit: var(--wmui-accent);
    --frb-submit-border: var(--wmui-accent);
    --frb-submit-hover: #447ff5;
    --frb-focus: var(--wmui-accent);
    --frb-error: var(--wmui-red);
    --frb-katie-gold: #ffcc00;
}

/* Hide when editing */
.action-edit #centralNotice,
.ve-activated #centralNotice {
    display: none !important;
}

/* Fix fixed position z-index for de.wikipedia and &quot; , &quot;'&quot; , &quot;gesproken_wiki&quot; , &quot;'&quot; , &quot; element on nl.wikipedia */
.mw-body { z-index: auto; }
#siteNotice { z-index: 3; }

/* Border-Box */

.frb,
.frb *,
.frb *:before,
.frb *:after {
    -moz-box-sizing: border-box;
    -webkit-box-sizing: border-box;
    box-sizing: border-box;
}

/* Banner wide settings */

.frb input,
.frb button {
    font-size: inherit;
    font-family: inherit;
}

.frb button {
    cursor: pointer;
    border: 0;
    background: transparent;
    padding: 0;
}

.frb frb-amt,
.frb-replaced {
    white-space: nowrap;
}

@media (prefers-reduced-motion: reduce) {
    .frb,
    .frb * {
        transition-duration: 0.01ms !important;
    }
}

/* --- Main banner wrapper --- */

.frb {
    display: none;
    background-color: var(--wmui-base100);
    color: var(--frb-body);
    font-size: 16px;
    line-height: 1.1875; /*19px @16px*/
    font-family: system-ui, -apple-system,BlinkMacSystemFont, &quot;Segoe UI&quot;, Roboto, &quot;Oxygen-Sans&quot;, Ubuntu, Cantarell, Lato, &quot;Helvetica Neue&quot;, Helvetica, Arial, sans-serif;
    text-align: left; /* needed because of #siteNotice { text-align: center; } in MediaWiki */
    font-weight: normal;
    font-style: normal; /* needed for uk.wikipedia */
}

body.rtl .frb {
    text-align: right;
}

.frb-in-article {
    margin-bottom: 20px;
}

.frb-nag,
.frb-fixed {
    position: fixed;
    bottom: 0;
    left: 0;
    width: 100%;
    padding: 16px;
    background-color: var(--wmui-base100);
    border-top: 1px solid var(--wmui-base70);
    box-shadow: 0 -1px 1px rgba(0,0,0,0.1);
}

/* Avoid overlapping Vector 2022&quot; , &quot;'&quot; , &quot;s fixed width toggle */
@media ( min-width: 1400px ) {
    body.skin-vector-2022 .frb-nag,
    body.skin-vector-2022 .frb-fixed {
        left: 48px;
        right: 48px;
        width: auto;
        border-left: 1px solid var(--wmui-base70);
        border-right: 1px solid var(--wmui-base70);
    }
}

.frb-layout {
    display: grid;
    grid-template-columns: auto 330px;
    grid-template-rows: 420px auto;
    grid-template-areas:
        &quot;main sidebar&quot;
        &quot;footer sidebar&quot;;
}

@media (max-width: 959px) {
    .frb-layout {
        grid-template-rows: auto auto;
    }
}

/* --- Icon buttons --- */

.frb .frb-icon-btn {
    display: block;
    cursor: pointer;
    background-repeat: no-repeat;
    background-position: center;
    opacity: .55;
}
.frb .frb-icon-btn:hover {
    opacity: 1;
}

.frb .frb-close {
    position: absolute;
    top: 0;
    right: 0;
    width: 45px;
    height: 45px;
    background-image: url(&quot;data:image/svg+xml,%3Csvg viewBox=&quot; , &quot;'&quot; , &quot;0 0 10 10&quot; , &quot;'&quot; , &quot; xmlns=&quot; , &quot;'&quot; , &quot;http://www.w3.org/2000/svg&quot; , &quot;'&quot; , &quot;%3E%3Cg stroke=&quot; , &quot;'&quot; , &quot;%23000&quot; , &quot;'&quot; , &quot; fill=&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot; stroke-linecap=&quot; , &quot;'&quot; , &quot;round&quot; , &quot;'&quot; , &quot;%3E%3Cpath d=&quot; , &quot;'&quot; , &quot;M1 1 l8 8 M9 1 l-8 8&quot; , &quot;'&quot; , &quot;%3E%3C/path%3E%3C/g%3E%3C/svg%3E&quot;);
    background-size: 16px 16px;
}
.frb .frb-step.frb-step-1 .frb-close {
    top: -15px;
    right: -15px;
}
body.rtl .frb .frb-close {
    right: auto;
    left: 0;
}
.frb-nag-layout .frb-close {
    top: -17px;
    right: -17px;
}

.frb .frb-back {
    position: absolute;
    top: 0;
    left: 0;
    width: 45px;
    height: 45px;
    background-image: url(&quot;data:image/svg+xml,%3Csvg xmlns=&quot; , &quot;'&quot; , &quot;http://www.w3.org/2000/svg&quot; , &quot;'&quot; , &quot; viewBox=&quot; , &quot;'&quot; , &quot;0 0 20 16&quot; , &quot;'&quot; , &quot;%3E%3Cg fill=&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot; fill-rule=&quot; , &quot;'&quot; , &quot;evenodd&quot; , &quot;'&quot; , &quot; transform=&quot; , &quot;'&quot; , &quot;translate(1 1)&quot; , &quot;'&quot; , &quot;%3E%3Cpath stroke=&quot; , &quot;'&quot; , &quot;%23000&quot; , &quot;'&quot; , &quot; stroke-linecap=&quot; , &quot;'&quot; , &quot;round&quot; , &quot;'&quot; , &quot; stroke-linejoin=&quot; , &quot;'&quot; , &quot;round&quot; , &quot;'&quot; , &quot; stroke-width=&quot; , &quot;'&quot; , &quot;1.778&quot; , &quot;'&quot; , &quot; d=&quot; , &quot;'&quot; , &quot;M7.181 13.285L.753 7 7.181.715&quot; , &quot;'&quot; , &quot;%3E%3C/path%3E%3Crect fill=&quot; , &quot;'&quot; , &quot;%23000&quot; , &quot;'&quot; , &quot; width=&quot; , &quot;'&quot; , &quot;18.182&quot; , &quot;'&quot; , &quot; height=&quot; , &quot;'&quot; , &quot;1.778&quot; , &quot;'&quot; , &quot; x=&quot; , &quot;'&quot; , &quot;.818&quot; , &quot;'&quot; , &quot; y=&quot; , &quot;'&quot; , &quot;6.111&quot; , &quot;'&quot; , &quot; rx=&quot; , &quot;'&quot; , &quot;.889&quot; , &quot;'&quot; , &quot;%3E%3C/rect%3E%3C/g%3E%3C/svg%3E&quot;);
    background-size: 20px 13px;
}
body.rtl .frb .frb-back {
    left: auto;
    right: 0;
    transform: rotate(180deg);
}

/* --- RML Back button --- */

.back-rml, .frb-nag .frb-rml-close-wrapper {
    display: none;
}
.frb-nag .back-rml {
    text-align: center;
    margin: 0 auto;
    padding: 2px;
    font-size: 11px;
    line-height: 1;
    text-transform: uppercase;
    cursor: pointer;
    color: var(--frb-muted);
    height: 45px;
}
.frb-nag .back-rml:hover {
    color: var(--frb-muted-hover);
}

.frb-rml-displayed .back-rml {
    display: block;
}

/* --- RML Close button --- */

.frb-rml-close-wrapper {
    text-align: center;
}

.frb-rml-close-wrapper button {
    font-size: 12px;
    color: var(--frb-muted);
    height: 45px;
}

.frb-rml-close-wrapper:hover button {
    color: var(--frb-muted-hover);
}

.frb-rml-close-icon {
    width: 10px;
    height: 10px;
    margin-bottom: -1px;
}

.frb-rml-close-icon g {
    stroke: currentColor;
}

/* -------------- Message -------------- */

.frb-message {
    grid-area: main;
    display: flex;
    flex-direction: column;
    justify-content: center;
    position: relative;
    border-radius: .75em;
    background: #308557;
    border: 6px solid #308557;
    color: var(--wmui-base100);
    font-weight: normal;
    font-size: 14px;
    line-height: 1.6; /*24px @15px*/
    z-index: 1;
    padding: 20px;
}

@media (min-width: 860px) {
    .frb-message {
        padding: 10px 20px 20px;
    }
}

@media (min-width: 960px) {
    .frb-message {
        font-size: 1.175vw;
    }
}

@media (min-width: 1200px) {
    .frb-message {
        font-size: 1.1vw;
    }
}

@media (min-width: 1400px) {
    .frb-message {
        font-size: 1vw;
    }
}

@media (min-width: 1800px) {
    .frb-message {
        font-size: 18px;
    }
}

.frb-message-icon {
    float: left;
    margin-top: 4px; /*in px since margin is consistent on all bp*/
    margin-right: 2px;
    height: 1em;
    width: 1em;
}

.frb-message-icon circle {
    fill: #FEFD34;
}

.frb-nag .frb-message {
    border: 6px solid var(--frb-message-border);
}

.frb-nag .frb-message-icon circle {
    fill: var(--frb-message-border);
}
.frb-message-icon path {
    fill: var(--wmui-base0);
}

.frb-nag .frb-message-icon path {
    fill: var(--wmui-base100);
}

.frb-nag .frb-message-icon {
    margin-top: 3px;
}

@media all and (min-width: 1300px) {
    .frb-nag .frb-message-icon {
        margin-top: 4px;
    }
}

body.rtl .frb-message-icon {
    float: right;
    margin-right: 0;
    margin-left: 4px;
}

.frb-greeting .frb-message-icon {
    float: none;
    margin-right: 0;
    margin-top: 0;
    margin-bottom: -2px;
}

.frb-message p {
    margin: 0 0 1em;
}

.frb-message p:last-child {
    margin: 0;
}

.frb-greeting {
    flex: 0 0 auto;
    max-height: 62px;
    margin-bottom: 0.5em;
    text-align: center;
    font-size: 1.75em;
    line-height: 1;
    font-weight: bold;
}

.frb-subgreeting {
    font-size: 0.6em;
    line-height: 1.6;
    font-weight: normal;
}

.frb-message-content {
    padding-bottom: 25px;
}

@media (max-width: 959px) {
    .frb-greeting {
        max-height: none;
    }
    .frb-message-content {
        font-size: 14px !important;
    }
}

@media (max-width: 860px) {
    .frb-message-content {
        padding-bottom: 55px;
    }
}


/* Nag styles */

.frb-nag {
    cursor: pointer;
}

.frb-nag-layout {
    display: grid;
    grid-template-areas: &quot;main sidebar&quot;;
    grid-template-columns: auto 360px;
}

@media (min-width: 1200px) {
    .frb-nag-layout {
        grid-template-columns: auto 440px;
    }
}

.frb-nag .frb-message {
    padding: 22px 26px;
    background: var(--frb-message-background);
    color: var(--frb-message);
}

@media (max-width: 1400px) {
    .frb-nag .frb-message {
        padding: 18px 18px;
        font-size: 16px;
    }
}

@media (max-width: 1100px) {
    .frb-nag .frb-message {
        font-size: 14px;
    }
}

.frb-nag .frb-form-wrapper {
    padding: 0 16px;
    display: flex;
    align-items: center;
    justify-content: center;
}

/* Triangle */
.frb-message::after {
    position: absolute;
    content: &quot; &quot;;
    top: 180px;
    right: -25px;
    margin: -10px 0 0 0;
    border: 10px solid transparent;
    border-left-color: #308557;
    pointer-events: none;
}
body.rtl .frb-message::after {
    right: auto;
    left: -25px;
    border-left-color: transparent;
    border-right-color: #308557;
}

.frb-nag .frb-message::after {
    top: 50%;
    border-left-color: var(--frb-message-border);
}

body.rtl .frb-nag .frb-message::after {
    border-left-color: transparent;
    border-right-color: var(--frb-message-border);
}

/* -------------- Form -------------- */

.frb-form-wrapper {
    grid-area: sidebar;
    position: relative;
    background: var(--wmui-base100);
}

.frb-form-wrapper fieldset {
    border: 0;
    margin: 0 auto;
    padding: 0 0 6px 0;
}

.frb-form-wrapper .frb-amounts {
    padding: 0;
    margin-top: 8px;
}

.frb-form-wrapper legend,
.frb-rml-form-legend {
    display: block;
    margin: 0 0 2px;
    padding: 0 4px;
    font-weight: normal;
    text-align: inherit;
    font-size: 14px;
    line-height: 1.2142857143; /*17px @14px*/
    color: var(--frb-muted);
    transition: all .25s ease-in-out;
}

.frb-form-wrapper {
    counter-reset: count;
}
.frb-numbered {
    counter-increment: count;
}
.frb-numbered::before {
    content: counter(count) &quot;. &quot;;
    position: absolute;
    left: -12px;
}
body.rtl .frb-numbered::before {
    left: auto;
    right: -12px;
}

.frb-rml-form-legend {
    padding: 0 0 2px;
}

.frb-frequency legend,
.frb-amounts legend {
    padding: 0 5px;
}

.frb-form-wrapper fieldset:first-of-type legend {
    padding-top: 0;
}

.frb-form-wrapper ul {
    display: flex;
    flex-wrap: wrap;
    width: 100%;
    margin: 0;
    padding: 0;
    list-style: none;
}

.frb-form-wrapper li {
    display: inline-flex;
    justify-content: flex-start;
    align-items: center;
    margin: 0;
}

#frb-form {
    padding-left: 25px;
    position: relative;
    overflow: hidden;
    height: 100%; /* ensure varying height steps don&quot; , &quot;'&quot; , &quot;t get cut off */
}
body.rtl #frb-form {
    padding-left: 0;
    padding-right: 25px;
}

.frb-frequency ul li {
    flex: 1 0 0;
}

.frb-amounts ul li {
    flex: 0 0 32%;
    max-width: 32%;
}

.frb-amounts ul li.frb-amt-other {
    flex: 0 0 67%;
    max-width: 67%;
}

.frb-amounts .frb-radio-label {
    white-space: nowrap;
}

/* --- Common Button Styles --- */

/* Hide radio buttons */
.frb-form-wrapper .frb-methods input[type=&quot;radio&quot;],
.frb-form-wrapper .frb-optin input[type=&quot;radio&quot;],
.frb-form-wrapper input[type=&quot;checkbox&quot;] {
    position: absolute;
    overflow: hidden;
    height: 1px;
    width: 1px;
    clip: rect(0 0 0 0);
    border: 0;
    margin: -1px;
    padding: 0;
}

/* TODO: are these frb-btn styles needed? */
.frb-btn {
    width: 100%;
    height: 48px;
    display: block;
    background-color: var(--frb-button);
    color: var(--frb-body);
    font-size: 16px;
    line-height: 1.25; /*20px @16px*/
    padding: 13px 4px 15px 4px;
    border: 1px solid var(--frb-button-border);
    border-radius: 2px;
    outline: 0;
    text-align: center;
    cursor: pointer;
    font-family: inherit;
    font-weight: 500;
    transition: all .25s ease-in-out;
}
.frb-btn:hover {
    background-color: var(--wmui-base100);
    color: #444;
    border-color: var(--wmui-base50);
}
.frb-btn:active {
    background-color: #d9d9d9;
    color: var(--wmui-base0);
    border-color: #7d8389;
}
.frb-form-wrapper input[type=radio]:checked + .frb-btn {
    background-color: var(--frb-primary-dark);
    color: var(--wmui-base100);
    border-color: #7d8389;
}

.frb-rml-link {
    display: block;
    font-size: 16px;
    line-height: 1.125; /*18px @16px*/
    color: var(--frb-link);
    margin: 16px auto 0;
    text-align: center;
    font-weight: bold;
}

.frb-rml-link:hover,
.frb-rml-link:focus {
    color: var(--frb-link-hover);
}

.frb-radio,
.frb-radio-label {
    font-size: 16px;
    line-height: 1.375; /*22px @16px*/
}

.frb-radio {
    cursor: pointer;
    margin: 0 0 0 7px;
}

.frb-radio-label {
    display: block;
    padding: 12px 7px;
    cursor: pointer;
    font-weight: bold;
    flex: 1 0 auto;
}

/* Focus styles */

/*Outline reset*/
.frb-form-wrapper input[type=radio]:focus,
.frb-radio:focus + .frb-radio-label,
#frb-amt-other-input:focus,
#frb-rml-email:focus {
    outline: 0;
}

.frb button:focus,
.frb-btn:focus,
.frb-icon-btn:focus,
.frb-btn-submit:focus,
.frb-form-wrapper input[type=radio]:focus + .frb-btn,
.frb-form-wrapper input[type=radio]:focus + #frb-amt-other-label,
#frb-amt-other-input:focus,
.frb-rml-displayed .frb-rml-form input:focus,
.frb-nag-btn:focus,
#nag-rml-btn:focus {
    outline: 0;
    border-color: var(--frb-focus) !important;
    box-shadow: inset 0 0 0 2px var(--frb-focus);
}

.frb button.frb-submit:focus {
    box-shadow: inset 0 0 0 1px var(--frb-submit-hover), inset 0 0 0 2px var(--wmui-base100);
}

.frb-rml-displayed .frb-rml-form input:focus,
.frb-rml-displayed .frb-rml-form .frb-btn-submit:focus {
    position: relative;
}

.frb-radio:enabled:focus + .frb-radio-label,
.frb-radio:enabled:hover + .frb-radio-label {
    color: var(--frb-link);
    text-decoration: underline;
}

.frb-radio:disabled + label {
    opacity: 0.4;
    cursor: default;
}

#frb-amt-other-input:focus,
#frb-amt-other-input:hover {
    box-shadow: none;
    box-shadow: 0 2px #fff, 0 3px var(--frb-link);
    color: var(--frb-link);
}

.frb-form-wrapper input[type=radio]:focus + .frb-btn,
.frb-form-wrapper input[type=radio]:focus + #frb-amt-other-label,
.frb-form-wrapper input[type=radio]:checked + .frb-btn:focus,
.frb-form-wrapper input[type=radio]:checked + #frb-amt-other-label:focus,
.frb-form-wrapper .frb-btn-submit:focus,
#nag-yes-btn:focus {
    box-shadow: inset 0 0 0 1px var(--frb-submit), inset 0 0 0 2px var(--wmui-base100);
}

.frb-btn img {
    padding: 0 4px;
    max-width: 100%;
}

.frb-methods .frb-btn {
    height: 64px;
    line-height: 1.125; /*18px @16px*/
}

.frb-methods svg {
    max-width: 100%;
    width: 64px;
}

/*Slight adaption for Paypal logo with USD string*/
.frb-methods .frb-logo-payments--paypal-usd {
    width: 85px;
    margin-bottom: -6px;
}

/* -- Credit card logos -- */

.frb-cc-logo-wrapper {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    margin: 0 auto;
    max-width: 80px;
    font-size: 0; /* Remove spacing between icons */
}

.frb-pm-cc svg {
    flex: 0 0 24px;
    max-width: 24px;
    width: 24px;
    max-height: 15px; /* height needed for IE11 */
    margin: 2px;
    display: none;
}

/* Reduce card logo spacing/sizing when there are 4 methods */
.frb-payment-options .frb-button:first-child:nth-last-child(4) .frb-cc-logo-wrapper,
.frb-payment-options .frb-button:first-child:nth-last-child(4) ~ .frb-button .frb-cc-logo-wrapper {
    width: 61px;
}

.frb-payment-options .frb-button:first-child:nth-last-child(4) .frb-cc-logo-wrapper svg,
.frb-payment-options .frb-button:first-child:nth-last-child(4) ~ .frb-button .frb-cc-logo-wrapper svg {
    width: 24px;
    height: 15px;
}

/* Countries with 3 card types */
.frb-cctypes-vma .frb-cc-logo-wrapper {
    width: 100%;
    flex-wrap: nowrap;
}
.frb-cctypes-vma svg  {
    flex: 0 0 28%;
    max-width: 28%;
    width: 28%;
    max-height: 34px;
}

.frb-cc-logo-wrapper {
    display: none;
}

.frb-cctypes-vmad .frb-cc-logo-wrapper,
.frb-cctypes-vmaj .frb-cc-logo-wrapper,
.frb-cctypes-vma  .frb-cc-logo-wrapper,
.frb-cctypes-vm   .frb-cc-logo-wrapper {
    display: flex;
}

.frb-cctypes-vmad .frb-pm-cc-label,
.frb-cctypes-vmaj .frb-pm-cc-label,
.frb-cctypes-vma  .frb-pm-cc-label,
.frb-cctypes-vm   .frb-pm-cc-label {
    display: none;
}

.frb-cctypes-vmad .frb-cc-logo-visa,
.frb-cctypes-vmad .frb-cc-logo-mastercard,
.frb-cctypes-vmad .frb-cc-logo-amex,
.frb-cctypes-vmad .frb-cc-logo-discover,

.frb-cctypes-vmaj .frb-cc-logo-visa,
.frb-cctypes-vmaj .frb-cc-logo-mastercard,
.frb-cctypes-vmaj .frb-cc-logo-amex,
.frb-cctypes-vmaj .frb-cc-logo-jcb,

.frb-cctypes-vma  .frb-cc-logo-visa,
.frb-cctypes-vma  .frb-cc-logo-mastercard,
.frb-cctypes-vma  .frb-cc-logo-amex,

.frb-cctypes-vm   .frb-cc-logo-visa,
.frb-cctypes-vm   .frb-cc-logo-mastercard {
    display: inline-block;
}

/* Submit/Continue buttons (blue background) */
.frb .frb-btn-submit {
    width: 100%;
    display: block;
    margin-top: 6px;
    padding: 8px;
    color: var(--wmui-base100);
    background-color: var(--frb-submit);
    border-color: var(--frb-submit);
    cursor: pointer;
    border: 0;
    border-radius: 2px;
    font-size: 16px;
    font-weight: bold;
    transition: all .25s ease-in-out;
    height: 45px;
}
.frb .frb-btn-submit:hover {
    background-color: var(--frb-submit-hover);
    border-color: var(--frb-submit-hover);
}
.frb .frb-btn-submit:active {
    background-color: var(--frb-primary-dark);
    border-color: var(--frb-primary-dark);
    box-shadow: none;
}

.frb-submit-txt-once { display: inline; }
.form-monthly .frb-submit-txt-once { display: none; }

.frb-submit-txt-monthly { display: none; }
.form-monthly .frb-submit-txt-monthly { display: inline; }

/* --- Other Amount --- */

/* TODO: Can we cut these down at all? */
#frb-amt-other-input::-webkit-input-placeholder {
    color: var(--frb-body);
}
#frb-amt-other-input::-moz-placeholder {
    opacity: 1;
    color: var(--frb-body);
}
#frb-amt-other-input:-ms-input-placeholder {
    color: var(--frb-body);
}
#frb-amt-other-input:-moz-placeholder {
    opacity: 1;
    color: var(--frb-body);
}

#frb-amt-other-input:focus::-webkit-input-placeholder {
    color: #666;
}
#frb-amt-other-input:focus::-moz-placeholder {
    opacity: 1;
    color: #666;
}
#frb-amt-other-input:focus:-ms-input-placeholder {
    color: #666;
}
#frb-amt-other-input:focus:-moz-placeholder {
    opacity: 1;
    color: #666;
}

#frb-amt-other-input:hover::-webkit-input-placeholder {
    color: var(--frb-link);
}
#frb-amt-other-input:hover::-moz-placeholder {
    opacity: 1;
    color: var(--frb-link);
}
#frb-amt-other-input:hover:-ms-input-placeholder {
    color: var(--frb-link);
}
#frb-amt-other-input:hover:-moz-placeholder {
    opacity: 1;
    color: var(--frb-link);
}

.frb-amt-other {
    width: 66.666666%;
}

#frb-amt-other-input {
    width: calc(100% - 33px);
    padding: 4px 2px 4px 7px;
    border: none;
    direction: ltr;
    text-align: left;
    font-size: 16px;
    font-family: inherit;
    font-weight: bold;
    color: var(--frb-body);
    box-shadow: 0 2px #fff, 0 3px var(--frb-body);
    -webkit-appearance: none;
    border-radius: 0; /* needed for iPad */
}

/* TODO: What is this for? */
@media all and (min-width: 1280px) {
    .frb-amt-other #frb-amt-other-input {
        margin: 0;
    }
}

body.rtl #frb-amt-other-input {
    text-align: right;
}

/* --- Transaction fees options --- */

/* Checkbox styles */

.frb-checkbox-label {
    position: relative;
    display: inline-block;
    margin: 13px 0 0 5px;
    width: calc(100% - 10px);
    padding-left: 26px;
    padding-top: 2px;
    cursor: pointer;
    font-size: 14px;
    line-height: 1.3571428571; /*19px @14px*/
    color: var(--frb-body);
}
body.rtl .frb-checkbox-label {
    margin: 13px 5px 0 0;
    padding-left: 0;
    padding-right: 26px;
}

/* Outer-box */
.frb-checkbox-label::before {
    position: absolute;
    content: &quot;&quot;;
    top: 3px;
    left: 0;
    display: inline-block;
    height: 17px;
    width: 17px;
    border-radius: 2px;
    border: 1px solid var(--wmui-base50);
    background-color: var(--wmui-base90);
}
body.rtl .frb-checkbox-label::before {
    left: auto;
    right: 0;
}

/* Checkmark */
.frb-checkbox-label::after {
    position: absolute;
    content: &quot;&quot;;
    top: 8px;
    left: 4px;
    display: inline-block;
    height: 5px;
    width: 9px;
    border-left: 2px solid;
    border-bottom: 2px solid;
    transform: rotate(-45deg);
    border-color: var(--wmui-base100);
}
body.rtl .frb-checkbox-label::after {
    left: auto;
    right: 4px;
}

.frb-ptf-total {
    font-weight: bold;
}
.frb-ptf-fee {
    white-space: nowrap;
}

/* Hide the checkmark by default */
.frb-checkbox + .frb-checkbox-label::after {
    content: none;
}
/* Unhide the checkmark on the checked state */
.frb-checkbox:checked + .frb-checkbox-label::after {
    content: &quot;&quot;;
}

.frb-checkbox:checked + .frb-checkbox-label:before {
    background-color: var(--wmui-accent);
    border-color: var(--wmui-accent);
}

/* Focus styles - unchecked */
.frb-checkbox:focus + .frb-checkbox-label::before {
    border-color: var(--wmui-accent);
    box-shadow: inset 0 0 0 1px var(--wmui-accent);
}

/* Focus styles - checked */
.frb-checkbox:focus:checked + .frb-checkbox-label::before {
    box-shadow: inset 0 0 0 1px var(--wmui-accent), inset 0 0 0 2px var(--wmui-base100);
}

/* Hover */
.frb-checkbox:hover + .frb-checkbox-label::before {
    background-color: var(--wmui-base80);
}

.frb-checkbox:checked:hover + .frb-checkbox-label:before {
    background-color: var(--frb-link-hover);
    border-color: var(--frb-link-hover);
}

/* --- Email opt-in --- */

.frb-form-wrapper .frb-optin {
    margin-bottom: 3px;
}

.frb-form-wrapper .frb-optin legend {
    margin-bottom: 8px;
    display: inline-block;
}

.frb-optin .frb-radio {
    margin: 6px 5px 0px 9px;
}

.frb-optin .frb-radio-label {
    float: right;
    width: calc(100% - 30px);
    white-space: normal;
    font-size: 14px;
    line-height: 1.3571428571; /*19px @14px*/
    font-weight: bold;
}

.frb-optin .frb-radio-label:hover,
.frb-optin .frb-radio:hover + .frb-radio-label {
    text-decoration: none;
}

.frb-optin-no-prompt {
    display: none;
    margin: 8px;
    padding: 4px 8px;
    border: 2px solid #900;
    border-radius: 2px;
    font-size: 14px;
    line-height: 1.2857142857; /*18px @14px*/
    font-weight: normal;
}

.frb-optin-no-prompt.is-positive {
    border-color: var(--wmui-green-dark);
    font-weight: bold;
}

.frb-optin-no-prompt__yes {
    display: none;
}

.frb-optin-no-prompt__no {
    display: block;
}

.frb-optin-no-prompt.is-positive .frb-optin-no-prompt__yes {
    display: block;
}

.frb-optin-no-prompt.is-positive .frb-optin-no-prompt__no {
    display: none;
}

.frb-optin-smallprint {
    padding: 8px 8px 0 8px;
}

/* --- Payment method Buttons --- */

/* Fade methods which aren&quot; , &quot;'&quot; , &quot;t monthly capable when monthly option is selected */
.form-monthly .no-monthly {
    opacity: 0.4 !important;
}

.form-monthly .no-monthly .frb-label {
    cursor: default;
}

.frb-form-wrapper .frb-methods {
    margin-top: 12px;
    padding-bottom: 0;
}

/* --- Footer / Small Print --- */
.frb-smallprint {
    font-size: 12px;
    line-height: 1.5; /*18px @12px*/
    color: var(--frb-muted);
    font-weight: normal;
}
.frb-smallprint a {
    color: var(--frb-muted);
    text-decoration: underline;
}
.frb-smallprint a:hover {
    color: var(--frb-muted-hover);
}

.frb-footer {
    grid-area: footer;
    padding-top: 8px;
    display: flex;
}

.frb-footer-message {
    flex: 1 1 100%;
    max-width: calc(100% - 160px);
    display: inline-block;
    padding-right: 45px;
}

.frb-footer-button {
    display: inline-flex;
    align-items: flex-start;
    flex: 0 0 160px;
    max-width: 160px;
    width: 160px;
    justify-content: flex-end;
    margin-top: 20px;
}

.frb-footer-button .frb-already-donated-button {
    font-size: 14px;
    font-weight: bold;
    display: inline-block;
    text-decoration: none;
    color: var(--frb-muted);
}
.frb-footer-button .frb-already-donated-button:hover {
    color: var(--wmui-base0);
}

@media (max-width: 1160px) {
    .frb-smallprint {
        font-size: 10px;
    }
}

/* --- Disable I already donated --- */
.frb.frb-iad-disabled .frb-iad {
    display: none;
}

/* --- Show and Hiding (Minimize and Maximize) --- */

.frb-nag-btns {
    display: flex;
    flex: 1 0 0;
}

.frb.frb-rml-displayed .frb-nag-btns {
    display: none;
}

button.frb-nag-btn {
    flex: 1 0 0;
    margin: 0 8px;
    padding: 6px;
    min-height: 48px;
    background-color: white;
    border: 1px solid var(--frb-button-border);
    border-radius: 2px;
    outline: 0;
    color: var(--frb-body);
    font-weight: normal;
    font-size: 13px;
    line-height: 1.3;
    transition: all 100ms;
    cursor: pointer;
}

@media all and (min-width: 1100px) {
    button.frb-nag-btn {
        font-size: 14px;
    }
}

@media all and (min-width: 1200px) {
    button.frb-nag-btn {
        font-size: 16px;
    }
}

button.frb-nag-btn:hover {
    transform: scale(1.043478);
}

#nag-yes-btn {
    font-weight: bold;
    background-color: var(--frb-submit);
    border: 1px solid var(--frb-submit);
    color: white;
}

/* --- Maybe later --- */

.frb-rml-disabled .frb-rml,
.frb-rml-disabled .frb-rml-link,
.frb-rml-disabled #nag-rml-btn {
    display: none;
}

#frb-main .frb-rml {
    position: absolute;
    width: 100%;
}

.frb-rml-form-wrapper {
    display: none;
    position: absolute;
    margin-top: 12px;
    margin-left: 12px;
    width: calc(100% - 12px);
    padding: 16px 16px 0 16px;
    text-align: left;
    background: var(--wmui-base100);
    border: 1px solid var(--wmui-base70);
    border-radius: 2px;
    box-shadow: 0 2px 2px 0 rgba(0, 0, 0, 0.15);
    z-index: 10;
}

.frb-rml-form-wrapper:after,
.frb-rml-form-wrapper:before {
    bottom: 100%;
    left: 50%;
    border: solid transparent;
    content: &quot; &quot;;
    height: 0;
    width: 0;
    position: absolute;
    pointer-events: none;
}

.frb-rml-form-wrapper:after {
    border-bottom-color: var(--wmui-base100);
    border-width: 10px;
    margin-left: -10px;
}

.frb-rml-form-wrapper:before {
    border-bottom-color: var(--wmui-base70);
    border-width: 11px;
    margin-left: -11px;
}


/* styles for bottom fixed banner */
.frb-fixed .frb-rml-form-wrapper:after,
.frb-fixed .frb-rml-form-wrapper:before {
    top: 100%;
    bottom: auto;
}

.frb-fixed .frb-rml-form-wrapper:after {
    border-top-color: var(--wmui-base100);
    border-bottom-color: transparent;
}

.frb-fixed .frb-rml-form-wrapper:before {
    border-top-color: var(--wmui-base70);
    border-bottom-color: transparent;
}

.frb-rml-form input {
    width: 100%;
    padding: 6px 8px 7px;
    border: 1px solid var(--wmui-base50);
    border-radius: 2px;
    color: var(--wmui-base0);
    font-size: 14px;
    height: 45px;
}

#frb-rml-email.frb-haserror {
    border-color: var(--frb-error) !important;
    box-shadow: inset 0 0 0 1px var(--frb-error) !important;
}

.frb-error-invalidemail {
    margin: 2px 0 0 !important;
}

.frb-rml-ty {
    text-align: left;
    font-size: 14px;
}

.frb-rml-displayed .frb-rml-form-wrapper {
    position: relative;
    z-index: 10;
    display: table !important;
    margin: -4px auto 0;
    padding: 0 12px;
    width: 100%;
    max-width: 340px;
    background: transparent;
    border: none;
    box-shadow: 0 0 0 0 rgba(0, 0, 0, 0);
}

.frb-rml-displayed .frb-rml-form-wrapper:after,
.frb-rml-displayed .frb-rml-form-wrapper:before {
    display: none !important;
}

.frb-rml-displayed .frb-rml {
    display: block !important;
    margin-top: 0;
}

.frb-rml-displayed .frb-rml-form legend {
   font-size: 12px;
   line-height: 1; /*12px @12px*/
   padding-bottom: 4px;
}

@media all and (min-width: 1200px) {
    .frb-rml-displayed .frb-rml-form legend {
       font-size: 14px;
       line-height: 1.2142857143; /*17px @14px*/
    }
}

.frb-rml-displayed .frb-rml-form input {
    display: inline;
    vertical-align: middle;
    width: 200px;
    height: 45px;
    padding: 7px 8px;
    margin: 0;
    border: 1px solid var(--wmui-base50);
    border-radius: 2px;
    color: var(--frb-body);
    direction: ltr;
    line-height: 1;
}

.frb-rml-displayed .frb-rml-form .frb-btn-submit {
    display: inline;
    vertical-align: middle;
    width: auto;
    height: 45px;
    margin-top: 0;
    margin-left: 2px;
    line-height: 1;
    padding: 9px 14px;
    font-size: 14px;
    border-radius: 2px;
}

.frb-prevent-page-jump {
    display: none;
}

/* -- Submit/&quot;Donate now&quot; button -- */

.frb .frb-submit {
    height: 52px;
    display: inline-block;
    cursor: default;
    margin: 4px 5px 0;
    padding: 5px 6px;
    width: calc(100% - 9px);
    background-color: var(--wmui-base100);
    border: 1px solid var(--wmui-base70);
    border-radius: 2px;
    color: rgba(84,89,93,0.2);
    font-weight: bold;
    transition: background-color 0.5s ease;
    word-break: keep-all; /* T259535 */
    line-height: 1.3;
}

.frb .frb-submit.active {
    background-color: var(--frb-submit);
    border-color: var(--frb-submit);
    color: var(--wmui-base100);
    cursor: pointer;
    opacity: 1;
}

.frb .frb-submit.active:hover {
    background-color: var(--frb-submit-hover);
    border-color: var(--frb-submit-hover);
}

.frb-submit-amount {
    display: none;
}
.frb-submit-label-monthly {
    display: none;
}
.frb-submit-label-now {
    display: inline;
}
.form-monthly .frb-submit-label-monthly {
    display: inline;
}
.form-monthly .frb-submit-label-now {
    display: none;
}
.frb-payment-options {
    display: flex;
    flex-wrap: wrap;
    width: 100%;
}

.frb-payment-options .frb-button {
    min-width: 33%;
}

/* For 4 payment options, one row */
.frb-payment-options .frb-button:first-child:nth-last-child(4),
.frb-payment-options .frb-button:first-child:nth-last-child(4) ~ .frb-button {
    min-width: 25%;
}

/* If there are 5 buttons, make the first one of the wider ones */
.frb-payment-options .frb-button:first-child:nth-last-child(5) {
    min-width: 50%;
}

.frb-methods .frb-button,
.frb-optin .frb-button {
    flex: 1 0 0;
    padding: 5px;
}

.frb-label {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 52px;
    padding: 0 4px;
    color: var(--frb-link);
    background-color: var(--frb-button);
    border: 1px solid var(--frb-button-border);
    border-radius: 2px;
    text-align: center;
    font-weight: bold;
    transition: all 0.5s ease;
    cursor: pointer;
}

.frb-pm-cc .frb-label {
    padding: 0 2px;
}

.frb-label:hover,
.frb-rml-email:hover {
    background-color: var(--frb-button-hover);
}

.form-monthly .no-monthly .frb-label:hover {
    background-color: var(--frb-button-hover);
}

.frb-radio:checked + .frb-label {
    background-color: var(--frb-button-selected);
    border-color: var(--frb-button-border-selected);
    color: var(--wmui-base100);
}

.frb-radio:focus + .frb-label,
#frb-rml-email:focus {
    box-shadow: inset 0 0 0 1px var(--frb-radio);
    border-color: var(--frb-radio);
}

.frb-radio:focus:checked + .frb-label {
    box-shadow: inset 0 0 0 1px var(--frb-button-selected), inset 0 0 0 2px var(--wmui-base100);
}

.frb-radio:checked + .frb-label .frb-logo-payments--paypal path,
.frb-radio:checked + .frb-label .frb-logo-payments--paypal-usd path,
.frb-radio:checked + .frb-label .frb-logo-payments--amazon path,
.frb-radio:checked + .frb-label .frb-logo-payments--applepay path,
.frb-radio:checked + .frb-label .frb-logo-payments--venmo path {
    fill: var(--wmui-base100);
}

/* Error messages */
.frb-error {
    display: none;
    margin: 5px 0 5px 5px;
    font-size: 14px;
    font-weight: 500;
    line-height: 1.3;
    color: var(--frb-error);
}

.frb-form-wrapper fieldset.frb-haserror .error-highlight,
.frb-form-wrapper fieldset.frb-haserror legend::before {
    color: var(--frb-error);
    font-weight: bold;
}

/* STEP 2 UPSELL*/

.frb-upsell,
.frb-step-monthly-diff-amt .frb-amt-monthly {
    transition: background-color 0.5s ease;
    padding: 10px 4px;
    text-align: center;
}

.frb-step-monthly-diff-amt .frb-amt-monthly {
    display: block;
    padding: 0 4px 10px 4px;
}

.frb-upsell-cta,
.frb-upsell-ty {
    font-size: 17px;
    line-height: 1.3;
    font-weight: bold;
    text-align: center;
}

.frb-upsell-color,
.frb-step-monthly-diff-amt .frb-amt-monthly label {
    display: block;
    font-size: 15px;
    line-height: 1.3;
    font-weight: normal;
    margin: .5em 0;
}

.frb .frb-monthly-diff-amt-link {
    font-size: 15px;
    line-height: 1.3;
    color: var(--frb-link);
    margin: 8px 2px;
    padding: 12px 10%;
    text-align: center;
    cursor: pointer;
    font-weight: bold;
}

#frb-amt-monthly-other-input {
    position: relative;
    text-align: center;
    font-size: 18px;
    height: 45px;
}

/* Steps */
#frb-form .frb-step {
    position: absolute;
    top: 0;
    padding-top: 24px;
    width: 300px;
    margin-left: 100%; /* Start hidden */
    visibility: hidden; /* Prevent tabbing to inputs */
}
body.rtl #frb-form .frb-step {
    margin-left: 0;
    margin-right: 100%;
}

#frb-form .frb-step-1 {
    position: relative;
    margin-left: 0;
    padding-top: 0;
    visibility: visible;
}
body.rtl #frb-form .frb-step-1 {
    margin-right: 0;
}
#frb-form .frb-step-optin,
#frb-form .frb-step-upsell {
    padding-top: 45px;
}

/*
    Already Donated Modal
*/
.modal {
    display: none;
    position: absolute;
    z-index: 999;
    background: rgba(255,255,255,0.95);
    min-width: 100%;
    min-height: 100%;
    top: 0;
    left: 0;
}
.modal-container {
    width: 100%;
    max-width: 600px;
    margin: 0 auto;
    box-shadow: 0 0 10px rgba(0,0,0,0.15);
    color: #000;
    background-color: #FFF;
    padding: 40px;
    box-sizing: border-box;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%,-50%);
}
.modal-close-x {
    border: none;
    background: none;
    margin: 0;
    padding: 0;
    cursor: pointer;
    position: absolute;
    top: 24px;
    left: 24px;
    width: 22px;
    height: 18px;
    opacity: 0.5;
}

.modal-title-text {
    font-size: 17px;
    color: #262626;
    font-weight: bold;
    display: inline-block;
}
.modal-text {
    font-size: 15px;
    margin: 20px auto 0;
    line-height: 1.44;
    max-width: 530px;
}
.frb-modal-buttons {
    display: flex;
    justify-content: center;
    margin: 48px auto 0;
}
.modal-close-button {
    margin: 0 8px;
    padding: 10px 15px;
    display: block;
    border-radius: 2px;
    border: solid 1px #a2a9b1;
    background-color: #f8f9fa;
    width: 165px;
    font-family: Arial;
    font-size: 16px;
    font-weight: bold;
    color: #007de2;
    cursor: pointer;
}
.modal-close-button:hover{
    background-color: #eaf3ff;
}

/*
    Responsive
*/
@media (max-width: 1200px) {
    .frb-footer {
        display: block;
    }
    .frb-footer-message {
        display: block;
        padding-right: 0;
        max-width: 100%;
    }
    .frb-footer-button {
        display: block;
        margin: 16px auto 0;
        width: auto;
        text-align: center;
    }
    .frb-already-donated-text {
        font-size: 14px;
    }

}


/* wmf identity */

.frb-wmf-identity {
    position: absolute;
    bottom: 0;
    right: 50%;
    width: 100%;
    max-width: 100%;
    clear: both;
    color: var(--wmui-base100);
    display: flex;
    justify-content: center;
    transform: translateX(50%);
}

.frb-wmf-identity-logo {
    max-width: 100px;
    flex: 0 0 100px;
    display: flex;
    align-items: center;
    margin: 0;
}

.frb-wmf-identity-logo img {
    width: 100%;
    max-width: 100%;
    filter: invert(1);
}

@media (max-width: 859px) {
    .frb-wmf-identity {
        flex-direction: column;
        align-items: center;
        justify-content: flex-end;
    }
    .frb-wmf-identity-logo {
        flex: 0 0 auto;
    }
}

.frb-wmf-identity-message {
    flex: 0 1 auto;
    font-size: 14px;
    display: flex;
    align-items: center;
    padding-left: 20px;
    color: var(--wmui-base100);
}

.frb-wmf-identity-message p {
    margin: 0;
    font-size: 13px;
}

/* screen reader visibility */
.sr-only {
    border: 0 !important;
    clip: rect(1px, 1px, 1px, 1px) !important;
    -webkit-clip-path: inset(50%) !important;
        clip-path: inset(50%) !important;
    height: 1px !important;
    margin: -1px !important;
    overflow: hidden !important;
    padding: 0 !important;
    position: absolute !important;
    width: 1px !important;
    white-space: nowrap !important;
}

.frb-submit-txt-donate { display: none; }
.frb-submit-txt-once {display: inline; }
.frb-submit-txt-monthly { display: none; }
.form-monthly .frb-submit-txt-once { display: none; }
.form-monthly .frb-submit-txt-monthly { display: inline; }

.frb-icon-heart path {
    fill: #900;
}






    

        
            
        

        
            Thank you, dear donor!
        

        
            Your generosity helps keep Wikipedia and its sister sites thriving. Select &quot;hide appeals&quot; to suppress fundraising messages in this browser for a week, or go back to the appeal if you&quot; , &quot;'&quot; , &quot;re still interested in donating.
        

        
            Hide appeals for a week
            Back to appeal
        

    




    

        
            
                
                Wikipedia still can&quot; , &quot;'&quot; , &quot;t be sold.
                December 18: An important update for readers in the United States
            
            
                Please don&quot; , &quot;'&quot; , &quot;t close this, it’s just a 1-minute read. We&quot; , &quot;'&quot; , &quot;re sorry to interrupt, but it&quot; , &quot;'&quot; , &quot;s Monday, December 18, and it will soon be too late to help us in our year-end fundraiser. We ask you to reflect on the number of times you visited Wikipedia this year and if you&quot; , &quot;'&quot; , &quot;re able to give $2.75 to the Wikimedia Foundation. If everyone reading this gave just $2.75, we&quot; , &quot;'&quot; , &quot;d hit our goal in a few hours.

                In the age of AI, access to verifiable facts is crucial. Wikipedia matters more than ever as a reliable source for emerging technologies, and you. Your gift supports how readers use Wikipedia now, and how revolutionary new systems will utilize it tomorrow.

                Reflect on Wikipedia&quot; , &quot;'&quot; , &quot;s usefulness in your life, and if the knowledge you gained was valuable, please give $2.75. Every contribution helps: every edit, every gift counts.
            

            
               
                   
               
               
                   Proud host of Wikipedia and its sister sites
               
            
        

        

            

                

                    

                    
                        
                            How often would you like to donate?
                        
                        
                            
                                
                                One time
                            
                            
                                
                                Give monthly
                            
                        
                    

                    
                        
                            Please select an amount (USD)
                            The average donation in the United States is around $13.
                        
                        
                            
                                
                                $2.75
                            
                            
                                
                                $10
                            
                            
                                
                                $15
                            
                            
                                
                                $25
                            
                            
                                
                                $50
                            
                            
                                
                                $75
                            
                            
                                
                                $100
                            
                            
                                Other amount
                                
                                
                                Other
                            
                        
                        
                            
                            I&quot; , &quot;'&quot; , &quot;ll generously add a little to cover the transaction fees so you can keep 100% of my donation.
                        
                    

                    
                        
                            Please select a payment method
                        
                        

                            

                            
                                
                                
                                    Credit/Debit Card
                                    
                                        Visa

                                        MasterCard

                                        Amex

                                        JCB

                                        Discover
                                    
                                
                            

                            

                            

                            
                                
                                
                                    PayPal
                                
                            

                            
                                
                                
                                    
                                        Venmo
                                        
                                    
                                
                            

                            

                            
                                
                                
                                    
                                        Google Pay
                                        
                                    
                                
                            

                            

                        
                    

                    
                        
                            
                                Continue
                                
                                    Donate  one time
                                    Donate  monthly
                                
                            
                        
                    

                    Please select an amount (minimum $1)
                    We cannot accept donations greater than 25000 USD through our website. Please contact our major gifts staff at benefactors@wikimedia.org.
                    Please select a payment method

                    
                        Maybe later
                    

                

                

                    

                    
                        
                            Can we follow up and let you know if we need your help again? The support and advice we get from donors in the United States is priceless, but many donors don&quot; , &quot;'&quot; , &quot;t let us stay in touch. Will you commit today, this Monday, to staying in touch with the Wikimedia Foundation?
                        
                        
                            
                                
                                Yes
                            
                            
                                
                                No
                            
                        
                        
                            Sorry to hear that. We don&quot; , &quot;'&quot; , &quot;t email often; would you consider changing your mind?
                            Thanks for changing your mind! We’ll respect your inbox.
                        
                        
                            Your information is handled in accordance with our donor privacy policy, and each email you receive will include easy unsubscribe options.
                        
                    

                    
                        Continue
                    

                    Please select an email option

                

                

                    

                    
                        
                            
                            Almost done: Please, make it  monthly.
                        Monthly support is the best way to ensure that Wikipedia keeps thriving.
                    

                    

                        
                            No thanks! I&quot; , &quot;'&quot; , &quot;ll make a one-time donation of 
                        

                        
                            Yes, I&quot; , &quot;'&quot; , &quot;ll donate  each month
                        

                    

                    Yes, I&quot; , &quot;'&quot; , &quot;ll donate monthly, but for a different amount

                

                

                    

                    Thank you for your support!
                    
                        Enter your monthly donation amount
                        
                    
                    Please select an amount (minimum $1)
                    We cannot accept donations greater than 25000 USD through our website. Please contact our major gifts staff at benefactors@wikimedia.org.
                    
                        Donate  monthly
                    

                

            

            

        

                
                    
                        Thank you! We will send you a reminder email.
                    
                    
                        
                        
                        
                        
                        
                        
                        
                        

                        Send me an email reminder
                        
                        Submit

                        
                            Please enter a valid email address i.e. name@domain.com
                        

                    
                    
                        ← Back
                    

                    
                        
                            Close
                            
                                
                                    
                                
                            
                        
                    
                
            

        
            
Problems donating? |
Other ways to give |
Frequently asked questions |
We never sell your information. By submitting, you are agreeing to our donor privacy policy and to sharing your information with the Wikimedia Foundation and its service providers in the U.S. and elsewhere.
We never sell your information. By submitting, you are agreeing to our donor privacy policy. The Wikimedia Foundation is a nonprofit, tax-exempt organization.
We never sell your information. By submitting, you are agreeing to our donor privacy policy and to sharing your information with the Wikimedia Foundation and its service providers in the U.S. and elsewhere. The Wikimedia Foundation is a nonprofit, tax-exempt organization.
If you make a recurring donation, you will be debited by the Wikimedia Foundation until you notify us to stop. We’ll send you an email which will include a link to easy cancellation instructions.

            
                
                    I already donated
                
            
        

    




    
        
            
                
                Sorry to interrupt, but time will soon run out for you to give in 2023. Please, donate $2.75.
            
        
        
            
            
                No, but maybe later when I have more time
                Yes, I&quot; , &quot;'&quot; , &quot;ll donate $2.75
            
        
    



var frb = frb || {};

frb.HIDE_DURATION_CLOSE = 3600; // 1 hour
frb.HIDE_DURATION_RML = 604800; // 1 week

frb.show = function() {
    $(&quot; , &quot;'&quot; , &quot;.frb-in-article&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
};

frb.hide = function() {
    /* Hide the banner, and remove related event handlers */
    $(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;).hide();
    $(&quot; , &quot;'&quot; , &quot;.frb-prevent-page-jump&quot; , &quot;'&quot; , &quot;).hide();
    $(window).off(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;);
    $(&quot; , &quot;'&quot; , &quot;#toc a&quot; , &quot;'&quot; , &quot;).off(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;);
};

/**
 * Show the given step
 * Literally just move into place and make it visible,
 * there may be other stuff needed to prepare it.
 *
 * @param  {string} step - name of step
 */
frb.showStep = function( step ) {

    var d = frb.reduceMotion ? 0 : 300, // animation duration in ms
        posPrev, posNext, posCrnt;

    if ( $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).hasClass(&quot; , &quot;'&quot; , &quot;rtl&quot; , &quot;'&quot; , &quot;) ) {
        posPrev = { &quot; , &quot;'&quot; , &quot;margin-right&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;-110%&quot; , &quot;'&quot; , &quot; };
        posNext = { &quot; , &quot;'&quot; , &quot;margin-right&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot; };
        posCrnt = { &quot; , &quot;'&quot; , &quot;margin-right&quot; , &quot;'&quot; , &quot;: 0 };
    } else {
        posPrev = { &quot; , &quot;'&quot; , &quot;margin-left&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;-110%&quot; , &quot;'&quot; , &quot; };
        posNext = { &quot; , &quot;'&quot; , &quot;margin-left&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot; };
        posCrnt = { &quot; , &quot;'&quot; , &quot;margin-left&quot; , &quot;'&quot; , &quot;: 0 };
    }

    function movePrev( $el ) {
        $el.animate( posPrev, d, function() {
            $(this).css({ &quot; , &quot;'&quot; , &quot;visibility&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot; });
        });
    }

    function moveNext( $el ) {
        $el.animate( posNext, d, function() {
            $(this).css({ &quot; , &quot;'&quot; , &quot;visibility&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot; });
        });
    }

    function moveCrnt( $el ) {
        $el.css({ &quot; , &quot;'&quot; , &quot;visibility&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; }).animate( posCrnt, d );
    }

    if ( step === &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot; ) {
        moveCrnt( $(&quot; , &quot;'&quot; , &quot;.frb-step-1&quot; , &quot;'&quot; , &quot;) );
        moveNext( $(&quot; , &quot;'&quot; , &quot;.frb-step-optin, .frb-step-upsell, .frb-step-monthly-diff-amt&quot; , &quot;'&quot; , &quot;) );
    } else if ( step === &quot; , &quot;'&quot; , &quot;optin&quot; , &quot;'&quot; , &quot; ) {
        movePrev( $(&quot; , &quot;'&quot; , &quot;.frb-step-1&quot; , &quot;'&quot; , &quot;) );
        moveCrnt( $(&quot; , &quot;'&quot; , &quot;.frb-step-optin&quot; , &quot;'&quot; , &quot;) );
        moveNext( $(&quot; , &quot;'&quot; , &quot;.frb-step-upsell, .frb-step-monthly-diff-amt&quot; , &quot;'&quot; , &quot;) );
    } else if ( step === &quot; , &quot;'&quot; , &quot;upsell&quot; , &quot;'&quot; , &quot; ) {
        movePrev( $(&quot; , &quot;'&quot; , &quot;.frb-step-1, .frb-step-optin&quot; , &quot;'&quot; , &quot;) );
        moveCrnt( $(&quot; , &quot;'&quot; , &quot;.frb-step-upsell&quot; , &quot;'&quot; , &quot;) );
        moveNext( $(&quot; , &quot;'&quot; , &quot;.frb-step-monthly-diff-amt&quot; , &quot;'&quot; , &quot;) );
    } else if ( step === &quot; , &quot;'&quot; , &quot;monthly-diff-amt&quot; , &quot;'&quot; , &quot; ) {
        movePrev( $(&quot; , &quot;'&quot; , &quot;.frb-step-1, .frb-step-optin, .frb-step-upsell&quot; , &quot;'&quot; , &quot;) );
        moveCrnt( $(&quot; , &quot;'&quot; , &quot;.frb-step-monthly-diff-amt&quot; , &quot;'&quot; , &quot;) );
    } else {
        // console.log( &quot; , &quot;'&quot; , &quot;Invalid step: &quot; , &quot;'&quot; , &quot; + step );
    }

};

frb.toggleMonthly = function( monthly ) {
    if( monthly.type === &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot; ){
        monthly = monthly.checked;
    }
    if ( monthly ) {
        $(&quot; , &quot;'&quot; , &quot;#frb-frequency-monthly&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
        $(&quot; , &quot;'&quot; , &quot;#frb-monthly-checkbox&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
        $(&quot; , &quot;'&quot; , &quot;#frb-form&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;form-monthly&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.no-monthly input[type=radio]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
        $(&quot; , &quot;'&quot; , &quot;.no-monthly&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
        $(&quot; , &quot;'&quot; , &quot;#frb-form&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;form-monthly&quot; , &quot;'&quot; , &quot;);
        if( $( &quot; , &quot;'&quot; , &quot;.form-monthly .no-monthly input[type=radio]&quot; , &quot;'&quot; , &quot; ).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;) ) {
            $(&quot; , &quot;'&quot; , &quot;.form-monthly .no-monthly input[type=radio]&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;);
            frb.setMethod({});
        }
        $(&quot; , &quot;'&quot; , &quot;.frb-cta-label-monthly&quot; , &quot;'&quot; , &quot;).show();
    } else {
        $(&quot; , &quot;'&quot; , &quot;#frb-frequency-onetime&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
        $(&quot; , &quot;'&quot; , &quot;#frb-monthly-checkbox&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
        $(&quot; , &quot;'&quot; , &quot;#frb-form&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;form-monthly&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.no-monthly input[type=radio]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
        $(&quot; , &quot;'&quot; , &quot;.frb-cta-label-monthly&quot; , &quot;'&quot; , &quot;).hide();
    }
};

frb.rml = {

    post: function() {
        /* Create the iframe for the form and use it as the form&quot; , &quot;'&quot; , &quot;s target */
        var frameName = &quot; , &quot;'&quot; , &quot;remindFrame&quot; , &quot;'&quot; , &quot;;
        var $form = $(&quot; , &quot;'&quot; , &quot;#frb-rml-form&quot; , &quot;'&quot; , &quot;);
        if ( $(&quot;iframe[name=&quot; + frameName + &quot;]&quot;).length === 0 ) {
            var $iframe = $(&quot; , &quot;'&quot; , &quot;&lt;iframe style=&quot;display: none;&quot; name=&quot;&quot; , &quot;'&quot; , &quot; + frameName + &quot; , &quot;'&quot; , &quot;&quot;>&lt;/iframe>&quot; , &quot;'&quot; , &quot;);
            $form.attr(&quot; , &quot;'&quot; , &quot;target&quot; , &quot;'&quot; , &quot;, $iframe.attr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;));
            $form.after($iframe);
        }
        $form[0].submit();
    },

    getCurrentDate: function() {
        /* Get current date in correct format for Silverpop */
        var today = new Date();
        var dd = today.getDate();
        var mm = today.getMonth()+1; // January is 0!
        var yyyy = today.getFullYear();

        if( dd &lt; 10 ) {
            dd = &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot; + dd;
        }
        if( mm &lt; 10 ) {
            mm = &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot; + mm;
        }

        return mm+&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;+dd+&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;+yyyy;
    },

    init: function() {
        /* Prep the reminder form */

        var form = document.getElementById(&quot; , &quot;'&quot; , &quot;frb-rml-form&quot; , &quot;'&quot; , &quot;);

        form.rml_country.value    = mw.centralNotice.data.country;
        form.rml_language.value   = mw.centralNotice.data.uselang;
        form.rml_submitDate.value = frb.rml.getCurrentDate();
        form.rml_segment.value    = Math.floor((Math.random() * 100) + 1);

        $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).click(function() {
            form.rml_source.value = &quot; , &quot;'&quot; , &quot;B2324_121810_en6C_dsk_p1_lg_txt_169C&quot; , &quot;'&quot; , &quot;;
            if ($(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;).hasClass(&quot; , &quot;'&quot; , &quot;frb-fixed&quot; , &quot;'&quot; , &quot;)) {
                $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).css( &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).position().top - 195 );
            }
            else {
                $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).css( &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).position().top + 40 );
            }
            $(&quot; , &quot;'&quot; , &quot;.frb-rml-form-wrapper&quot; , &quot;'&quot; , &quot;).toggle();
            $(&quot; , &quot;'&quot; , &quot;#frb-rml-email&quot; , &quot;'&quot; , &quot;).focus();
            $(&quot; , &quot;'&quot; , &quot;.frb-rml-close-wrapper&quot; , &quot;'&quot; , &quot;).show();
            return false;
        });

        // Move the rml popup if it&quot; , &quot;'&quot; , &quot;s open when PTF or error gets shown
        $(&quot; , &quot;'&quot; , &quot;.frb-amounts input&quot; , &quot;'&quot; , &quot;).on( &quot; , &quot;'&quot; , &quot;click input change&quot; , &quot;'&quot; , &quot;, function() {
            $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).css( &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).position().top + 40 );
            setTimeout( function() {
                $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).css( &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).position().top + 40 );
            }, 400 );
        });

        $(&quot; , &quot;'&quot; , &quot;#frb-rml-submit&quot; , &quot;'&quot; , &quot;).click(function() {
            if ( mw.util.validateEmail( form.Email.value ) ) {
                frb.rml.post();
                $(&quot; , &quot;'&quot; , &quot;.frb-rml-form-wrapper form&quot; , &quot;'&quot; , &quot;).hide();
                $(&quot; , &quot;'&quot; , &quot;.frb-rml-ty&quot; , &quot;'&quot; , &quot;).show();
                window.setTimeout( function() {
                    frb.hide();
                }, 2500);
                frb.altSetHideCookie( &quot; , &quot;'&quot; , &quot;close&quot; , &quot;'&quot; , &quot;, frb.HIDE_DURATION_RML );
                return false;
            } else {
                $(&quot; , &quot;'&quot; , &quot;#frb-rml-email&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;).focus();
                $(&quot; , &quot;'&quot; , &quot;.frb-error-invalidemail&quot; , &quot;'&quot; , &quot;).show();
                return false;
            }
        });
    }

};

frb.showSidebarTooltip = function () {
    mw.loader.using( [ &quot; , &quot;'&quot; , &quot;oojs-ui-core&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mediawiki.Uri&quot; , &quot;'&quot; , &quot; ] ).done( function () {

        let menuPinned;
        if ( mw.config.get(&quot; , &quot;'&quot; , &quot;skin&quot; , &quot;'&quot; , &quot;) === &quot; , &quot;'&quot; , &quot;vector-2022&quot; , &quot;'&quot; , &quot; ) {
            menuPinned = $(&quot; , &quot;'&quot; , &quot;#vector-main-menu-pinned-container > #vector-main-menu&quot; , &quot;'&quot; , &quot;).length > 0;
        } else {
            menuPinned = true; // sidebar always visible on Legacy Vector
        }

        let $donateLink = $( &quot; , &quot;'&quot; , &quot;#n-sitesupport a&quot; , &quot;'&quot; , &quot; );
        $donateLink.attr( &quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, ( i, oldUrl ) => {
            let uri = new mw.Uri( oldUrl );
            uri.query.utm_source = &quot; , &quot;'&quot; , &quot;tooltipOnBannerClose&quot; , &quot;'&quot; , &quot;;
            return uri.toString();
        });

        let popup = new OO.ui.PopupWidget( {
            $content: $( &quot; , &quot;'&quot; , &quot;&lt;p>You can donate at any time from this menu.&lt;/p>&quot; , &quot;'&quot; , &quot; ),
            padded: true,
            autoclose: true,
            align: &quot; , &quot;'&quot; , &quot;forwards&quot; , &quot;'&quot; , &quot;,
            autoFlip: false,
            $floatableContainer: menuPinned ? $donateLink : $( &quot; , &quot;'&quot; , &quot;#vector-main-menu-dropdown&quot; , &quot;'&quot; , &quot; ),
            position: menuPinned ? &quot; , &quot;'&quot; , &quot;after&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;below&quot; , &quot;'&quot; , &quot;,
        } );

        popup.$element.css(&quot; , &quot;'&quot; , &quot;z-index&quot; , &quot;'&quot; , &quot;, 5); // Fix so it shows above header
        $( document.body ).append( popup.$element );
        popup.toggle( true );

        setTimeout( () => {
            popup.$element.fadeOut( frb.fadeDuration );
        }, 5000 );
    } );
};

/* jshint maxerr: 600 */
frb.amounts = frb.amounts || {};

// Hard minimum amounts that can be given
// From https://github.com/wikimedia/wikimedia-fundraising-SmashPig/blob/master/PaymentData/ReferenceData/CurrencyRates.php
// Updated 2023-01-27
frb.amounts.minimums = {
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : 1,
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : 1.35,
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : 1.45,
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : 1.56,
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : 0.81,
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : 0.92,
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : 6.88,
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : 365,
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : 3.40,
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : 10, // T309818
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : 128,
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : 4.31,
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : 9.92,
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : 4.36,
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : 22,
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : 4.55,
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : 37,
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : 17,
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : 5.19,
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : 183,
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : 825,
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : 4684,
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : 19,
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : 3.80,
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : 39,
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : 0.92
};

frb.amounts.options7 = {
    // Big English
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : [2.75, 10, 15, 25, 50, 75, 100],
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : [2.75, 10, 15, 25, 50, 75, 100],
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : [2.75, 10, 15, 25, 50, 75, 100],
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : [2.75, 10, 15, 25, 50, 75, 100],
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : [2, 10, 15, 25, 50, 75, 100],
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : [2, 10, 15, 25, 50, 75, 100]
    },
    // Others
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : [20, 100, 150, 200, 300, 500, 750],
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : [500, 1000, 2000, 4000, 5000, 7000, 10000],
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : [10, 35, 50, 100, 200, 300, 400],
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : [25, 300, 500, 1000, 1500, 3000, 5000],
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : [300, 1000, 1500, 2000, 3000, 5000, 10000],
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : [10, 30, 50, 100, 200, 300, 500],
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : [20, 100, 150, 200, 500, 750, 1000],
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : [10, 20, 50, 100, 200, 300, 500],
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : [50, 100, 250, 500, 1000, 1500, 2500],
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : [10, 50, 75, 100, 200, 300, 500],
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : [30, 50, 100, 200, 300, 500, 1000],
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : [50, 75, 150, 300, 500, 750, 1000],
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : [30, 50, 100, 200, 300, 500, 1000],
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : [10, 20, 30, 50, 100, 200, 300],
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : [200, 250, 500, 750, 1000, 1500, 2000],
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : [2000, 3000, 5000, 10000, 20000, 30000, 50000],
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : [10000, 15000, 25000, 50000, 100000, 150000, 200000],
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : [40, 70, 150, 250, 500, 700, 1000],
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : [10, 15, 25, 50, 100, 150, 200],
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : [100, 200, 300, 500, 1000, 1500, 2000],
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : [3, 5, 10, 25, 50, 100, 200]
};

// 5 amount options. Since 2020 6C, no longer used
frb.amounts.options5 = {
    // Big English
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : [2.75, 15, 20, 50, 100],
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : [2.75, 15, 20, 50, 100],
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : [2.75, 15, 20, 50, 100],
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : [2.75, 15, 20, 50, 100],
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : [2, 10, 20, 50, 100],
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : [2, 10, 20, 50, 100],
    // Others
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : [20, 100, 200, 500, 1000],
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : [500, 2500, 4000, 7000, 10000],
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : [10, 50, 200, 600, 1000],
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : [150, 500, 1000, 3000, 5000],
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : [300, 1500, 2000, 5000, 10000],
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : [10, 50, 100, 300, 500],
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : [20, 100, 200, 500, 1000],
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : [10, 50, 100, 300, 500],
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : [10, 50, 100, 200, 1000],
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : [30, 100, 200, 500, 1000],
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : [50, 150, 300, 750, 1000],
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : [30, 100, 200, 500, 1000],
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : [10, 30, 50, 100, 250],
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : [60, 200, 400, 1000, 2000],
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : [1500, 5000, 10000, 25000, 50000],
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : [7000, 20000, 50000, 150000, 200000],
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : [35, 100, 200, 750, 1000],
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : [10, 50, 150, 300, 700],
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : [70, 200, 400, 1500, 2000],
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : [3, 10, 25, 50, 100]
};

// &quot;Average&quot; donation
frb.amounts.averages = {
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : 13,
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : 12,
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : 11,
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : 12,
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : 6,
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : 8,
    // Others
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : 60,
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : 2500,
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : 229,
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : 800,
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : 75,
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : 150,
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : 85,
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : 150,
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : 65,
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : 25,
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : 780,
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : 10200,
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : 35000,
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : 140,
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : 30,
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : 525,
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : 25
};

// &quot; , &quot;'&quot; , &quot;If everyone gave X&quot; , &quot;'&quot; , &quot;. Mostly the same as first asks option.
frb.amounts.ifEveryone = {
    // Big English
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : 2.75,
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : 2.75,
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : 2.75,
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : 2.75,
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : 2,
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : 2
    },
    // Others
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : 20,
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : 500,
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : 25,
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : 300,
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : 20,
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : 30,
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : 30,
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : 175,
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : 1500,
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : 7000,
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : 40,
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : 100,
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : 5
};

// Minimum fee/PTF amounts. Default is 0.35.
// Updated 2018-07-05 based on Ppena&quot; , &quot;'&quot; , &quot;s feedback
// Updated 2019-05-21 to approx 0.35 USD equivalent
frb.amounts.feeMinimums = {
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : 2,
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : 100,
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : 1.2,
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : 4,
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : 35,
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : 1,
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : 3,
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : 1.35,
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : 7.5,
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : 1.5,
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : 3,
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : 5,
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : 2,
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : 32,
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : 255,
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : 1300,
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : 7.4,
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : 1.3,
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : 15.5
};

// If one-time amount &lt;= left amount, suggest right amount for monthly
// If changing these, please update spreadsheet
// https://docs.google.com/spreadsheets/d/1z36zi8EegPLAvR5FYAgwz8ywKZ50QNB82SpwpTdk-xQ/edit#gid=1258723967
frb.amounts.monthlySuggest = {
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : [ // also GBP
        [ 1.99, 0 ],
        [ 2.35, 1.40 ],
        [ 9, 1.75 ],
        [ 12, 2 ],
        [ 15, 2.5 ],
        [ 18, 3 ],
        [ 21, 3.5 ],
        [ 24, 4 ],
        [ 27, 4.5 ],
        [ 30, 5 ],
        [ 33, 5.5 ],
        [ 36, 6 ],
        [ 39, 6.5 ],
        [ 42, 7 ],
        [ 45, 7.5 ],
        [ 48, 8 ],
        [ 51, 8.5 ],
        [ 54, 9 ],
        [ 57, 9.5 ],
        [ 60, 10 ],
        [ 63, 10.5 ],
        [ 66, 11 ],
        [ 69, 11.5 ],
        [ 72, 12 ],
        [ 75, 12.5 ],
        [ 102, 17 ],
        [ 250, 25 ],
        [ 499, 50 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : [ // also CAD, AUD, NZD
        [ 2.74, 0 ],
        [ 9, 1.75 ],
        [ 12, 2 ],
        [ 15, 2.5 ],
        [ 18, 3 ],
        [ 21, 3.5 ],
        [ 24, 4 ],
        [ 27, 4.5 ],
        [ 30, 5 ],
        [ 33, 5.5 ],
        [ 36, 6 ],
        [ 39, 6.5 ],
        [ 42, 7 ],
        [ 45, 7.5 ],
        [ 48, 8 ],
        [ 51, 8.5 ],
        [ 54, 9 ],
        [ 57, 9.5 ],
        [ 60, 10 ],
        [ 63, 10.5 ],
        [ 66, 11 ],
        [ 69, 11.5 ],
        [ 72, 12 ],
        [ 75, 12.5 ],
        [ 102, 17 ],
        [ 250, 25 ],
        [ 499, 50 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : [
        [ 299, 0 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2100, 350 ],
        [ 2400, 400 ],
        [ 2700, 450 ],
        [ 3000, 500 ],
        [ 3300, 550 ],
        [ 3600, 600 ],
        [ 3900, 650 ],
        [ 4200, 700 ],
        [ 4500, 750 ],
        [ 4800, 800 ],
        [ 5100, 850 ],
        [ 5400, 900 ],
        [ 5700, 950 ],
        [ 6000, 1000 ],
        [ 6300, 1050 ],
        [ 6600, 1100 ],
        [ 6900, 1150 ],
        [ 7200, 1200 ],
        [ 7500, 1250 ],
        [ 10800, 1800 ],
        [ 18000, 3000 ],
        [ 50000, 6000 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : [
        [ 25, 0 ],
        [ 50, 25 ],
        [ 100, 30 ],
        [ 200, 50 ],
        [ 300, 70 ],
        [ 500, 90 ],
        [ 1000, 110 ],
        [ 2500, 250 ],
        [ 5000, 500 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : [
        [ 499, 0 ],
        [ 3000, 500 ],
        [ 6000, 1000 ],
        [ 9000, 1500 ],
        [ 12000, 2000 ],
        [ 18000, 3000 ],
        [ 24000, 4000 ],
        [ 30000, 5000 ],
        [ 36000, 6000 ],
        [ 42000, 7000 ],
        [ 48000, 8000 ],
        [ 54000, 9000 ],
        [ 60000, 10000 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : [
        [ 9, 0 ],
        [ 10, 5 ],
        [ 60, 10 ],
        [ 90, 15 ],
        [ 120, 20 ],
        [ 180, 30 ],
        [ 240, 40 ],
        [ 300, 50 ],
        [ 360, 60 ],
        [ 420, 70 ],
        [ 480, 80 ],
        [ 540, 90 ],
        [ 600, 100 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : [
        [ 29, 0 ],
        [ 30, 20 ],
        [ 50, 30 ],
        [ 100, 40 ],
        [ 300, 50 ],
        [ 450, 75 ],
        [ 600, 100 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2400, 400 ],
        [ 3000, 500 ],
        [ 3600, 600 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : [ // Also RON, PLN
        [ 9, 0 ],
        [ 30, 5 ],
        [ 50, 10 ],
        [ 120, 20 ],
        [ 180, 30 ],
        [ 240, 40 ],
        [ 300, 50 ],
        [ 360, 60 ],
        [ 420, 70 ],
        [ 480, 80 ],
        [ 540, 90 ],
        [ 600, 100 ],
        [ 900, 150 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : [ // Also NOK
        [ 19, 0 ],
        [ 20, 10 ],
        [ 120, 20 ],
        [ 180, 30 ],
        [ 300, 50 ],
        [ 450, 75 ],
        [ 600, 100 ],
        [ 750, 125 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2100, 350 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : [
        [ 49, 0 ],
        [ 180, 30 ],
        [ 300, 50 ],
        [ 600, 100 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2100, 350 ],
        [ 2400, 400 ],
        [ 3000, 500 ],
        [ 3600, 600 ],
        [ 4200, 700 ],
        [ 4800, 800 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : [
        [ 1999, 0 ],
        [ 2300, 1000 ],
        [ 2700, 1100 ],
        [ 3300, 1200 ],
        [ 4200, 1300 ],
        [ 5500, 1400 ],
        [ 9000, 1500 ],
        [ 10500, 1700 ],
        [ 16000, 2600 ],
        [ 20800, 3400 ],
        [ 26000, 4200 ],
        [ 31200, 5000 ],
        [ 38400, 6400 ],
        [ 55000, 8500 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : [
        [ 99, 0 ],
        [ 120, 50 ],
        [ 170, 65 ],
        [ 220, 70 ],
        [ 320, 75 ],
        [ 480, 85 ],
        [ 520, 90 ],
        [ 750, 125 ],
        [ 1050, 170 ],
        [ 1350, 225 ],
        [ 1600, 250 ],
        [ 1800, 300 ],
        [ 2100, 320 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : [
        [ 9, 0 ],
        [ 12, 5 ],
        [ 17, 6 ],
        [ 26, 7 ],
        [ 48, 8 ],
        [ 55, 9 ],
        [ 78, 13 ],
        [ 105, 17 ],
        [ 130, 21 ],
        [ 160, 26 ],
        [ 180, 30 ],
        [ 210, 32 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : [
        [ 9999, 0 ],
        [ 11300, 5000 ],
        [ 17000, 5200 ],
        [ 22000, 5500 ],
        [ 27000, 5800 ],
        [ 45000, 7500 ],
        [ 55000, 9000 ],
        [ 75000, 12500 ],
        [ 105000, 17000 ],
        [ 120000, 20000 ],
        [ 160000, 25000 ],
        [ 180000, 30000 ],
        [ 250000, 34000 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : [
        [ 9, 0 ],
        [ 12, 6 ],
        [ 22, 7 ],
        [ 35, 8 ],
        [ 45, 9 ],
        [ 55, 10 ],
        [ 80, 12 ],
        [ 105, 16 ],
        [ 160, 25 ],
        [ 210, 35 ],
        [ 270, 45 ],
        [ 320, 50 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : [
        [ 39, 0 ],
        [ 48, 25 ],
        [ 60, 28 ],
        [ 110, 30 ],
        [ 160, 35 ],
        [ 260, 45 ],
        [ 270, 50 ],
        [ 350, 60 ],
        [ 550, 85 ],
        [ 650, 90 ],
        [ 750, 120 ],
        [ 1500, 160 ],
        [ Infinity, 0 ]
    ]
    
};
frb.amounts.monthlySuggest.GBP = frb.amounts.monthlySuggest.EUR;
frb.amounts.monthlySuggest.CAD = frb.amounts.monthlySuggest.USD;
frb.amounts.monthlySuggest.AUD = frb.amounts.monthlySuggest.USD;
frb.amounts.monthlySuggest.NZD = frb.amounts.monthlySuggest.USD;

frb.amounts.monthlySuggest.RON = frb.amounts.monthlySuggest.MYR;
frb.amounts.monthlySuggest.PLN = frb.amounts.monthlySuggest.MYR;
frb.amounts.monthlySuggest.NOK = frb.amounts.monthlySuggest.DKK;

frb.currencyFormats = {
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;fr&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;£\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;cy&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ga&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;mt&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;nl&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€ \t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;lv&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€ \t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;tr&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€ \t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t €&quot; , &quot;'&quot; , &quot;
    },
    // Others
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t Kč&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t kr.&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t Ft&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;he&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t ₪&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;yi&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t ₪&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ar&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t ₪&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;₪ \t&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;₹ \t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;¥\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;RM\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t kr&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t zł&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t lei&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t kr&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;₴\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;R \t&quot; , &quot;'&quot; , &quot;,
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;R$\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;R$ \t&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;S/. \t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$U \t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t CHF&quot; , &quot;'&quot; , &quot;
};

// Check in user language first, then fall back to English
frb.countryNames = {
    &quot; , &quot;'&quot; , &quot;af&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Suid-Afrika&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;the United States&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Canada&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;the UK&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Ireland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Australia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;New Zealand&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Argentina&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Austria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Belgium&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Brazil&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Switzerland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Chile&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Colombia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;the Czech Republic&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Denmark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Spain&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;France&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Greece&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Hong Kong&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Hungary&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;India&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Italy&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Japan&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Luxembourg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Latvia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Mexico&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Malaysia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;the Netherlands&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Norway&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Peru&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Poland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Romania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Sweden&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Slovakia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Ukraine&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Uruguay&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;South Africa&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;ca&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;d’Àustria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Bèlgica&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Dinamarca&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;a Espanya&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;d’Hongria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;d’Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Letònia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Luxemburg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Malàisia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Noruega&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Polònia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Romania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;d’Eslovàquia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Sud-àfrica&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;d’Ucraïna&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;cs&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v České republice&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Rakousku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Belgii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Dánsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Řecku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Izraeli&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Lucembursku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Malajsii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Norsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Portugalsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ve Švédsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Jihoafrické republice&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;es&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Argentina&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Austria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Bélgica&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Brasil&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Chile&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Colombia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Dinamarca&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en España&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Hungría&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Luxemburgo&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Letonia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en México&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Malasia &quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Noruega&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Perú&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Polonia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Rumania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Eslovaquia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Ucrania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en los Estados Unidos&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Uruguay&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Sudafrica&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;da&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Østrig&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Belgien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Tjekkiet&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Danmark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Spanien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Grækenland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ungarn&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Letland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Luxembourg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Malaysia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Norge&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Polen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Rumænien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sverige&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Slovakiet&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sydafrika&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ukraine&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;nl&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Nederland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Oostenrijk&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in België&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Denemarken&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Tsjechië&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Spanje&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Griekenland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Hongarije&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Israël&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Letland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Luxemburg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Maleisië&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Noorwegen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Polen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Roemenië&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Zweden&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Slowakije&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Zuid-Afrika&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Oekraïne&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;fr&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Autriche&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Belgique&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Suisse&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;au Canada&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en République tchèque&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;au Danemark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Espagne&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en France&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Grèce&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Hongrie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Israël&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Lettonie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;au Luxembourg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Malaisie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Norvège&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Pologne&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;au Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Roumanie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Suède&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Slovaquie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Afrique du Sud&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Ukraine&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;de&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Österreich&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Belgien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in der Schweiz&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Tschechien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Dänemark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Spanien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Griechenland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Ungarn&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Lettland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Luxemburg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Malaysia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Norwegen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Polen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Rumänien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Schweden&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in der Slowakei&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Südafrika&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in der Ukraine&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;el&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Αυστρία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στο Βέλγιο&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Τσεχία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Δανία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Ισπανία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Ελλάδα&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Ουγγαρία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στο Ισραήλ&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Λετονία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στο Λουξεμβούργο&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Μαλαισία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Νορβηγία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Πολωνία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Πορτογαλία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Ρουμανία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Σουηδία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Σλοβακία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Νότια Αφρική&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Ουκρανία&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;he&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;אוסטרליה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;בלגיה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot;בצ&quot; , &quot;'&quot; , &quot;כיה&quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;דנמרק&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ספרד&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ביוון&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;הונגריה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ישראל&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;לטביה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;לוקסמבורג&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;מלזיה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;נורווגיה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;פולין&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;פורטוגל&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;רומניה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;בשוודיה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;סלובקיה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;דרום אפריקה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;אוקראינה&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;hu&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ausztriai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;belgiumi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;dániai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;spanyolországi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;magyarországi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;izraeli&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;lettországi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;luxemburgi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;malajziai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;norvégiai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;lengyelországi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;portugáliai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;romániai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;szlovákiai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;dél-afrikai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ukrajnai&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;it&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Italia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Svizzera&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;lv&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Austrijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Beļģijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Dānijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Spānijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Ungārijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Izraēlas valstī&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Latvijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Luksemburgā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Malaizijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Norvēģijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Polijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Portugālē&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Rumānijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Slovākijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Dienvidāfrikas valstī&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Ukrainā&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;nb&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Østerrike&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Belgia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Tsjekkia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Danmark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Spania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Hellas&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ungarn&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Latvia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Luxembourg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Malaysia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Norge&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Polen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Romania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sverige&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Slovakia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sør-Afrika&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ukraina&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;pl&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Austrii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Belgii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Danii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Hiszpanii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Węgrzech&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Izraelu&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Łotwie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Luksemburgu&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Malezji&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Norwegii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Polsce&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Portugalii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Rumunii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Słowacji&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Republice Południowej Afryki&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Ukrainie&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Áustria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Bélgica&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;no Brasil&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na República Checa&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Dinamarca&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Espanha&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Grécia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Hungria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;em Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Letónia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;no Luxemburgo&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Malásia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Noruega&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Polónia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;em Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Roménia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Suécia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Eslováquia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na África do Sul&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Ucrânia&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Austria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Belgia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Danemarca&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;în Spania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Ungaria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Latvia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Luxemburg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Malaezia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Norvegia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Polonia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Portugalia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din România&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Slovacia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Africa de Sud&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Ucraina&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Австрии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Бельгии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Дании&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Испании&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Венгрии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Израиле&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Латвии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Люксембурге&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Малайзии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Норвегии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Польше&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Португалии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Румынии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Словакии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Южной Африке&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Украине&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;sk&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Rakúsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Belgicku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Dánsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Španielsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Maďarsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Izraeli&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Lotyšsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Luxembursku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Malajzii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Nórsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Poľsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Portugalsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Rumunsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Slovensku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Juhoafrickej republike&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Ukrajine&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;sv&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sverige&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Österrike&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Belgien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Tjeckien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Danmark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Spanien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Grekland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ungern&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Lettland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Luxemburg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Malaysia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Norge&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Polen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Rumänien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Slovakien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sydafrika&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ukraina&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;uk&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Австрії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Бельгії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Данії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Іспанії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Угорщині&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Ізраїлі&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Латвії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Люксембургу&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Малайзії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Норвегії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Польщі&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Португалії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Румунії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Словаччині&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у ПАР&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Україні&quot; , &quot;'&quot; , &quot;
    }
};

/* 
Most of the translations are actually using &quot;in COUNTRY&quot; or similar to account for grammar differences.
So this makes English do the same, and allows us to use a clearer %in-country% variable, while avoiding
breaking old content using %country%.
*/
frb.inCountryNames = JSON.parse( JSON.stringify( frb.countryNames ) ); // deep copy
frb.inCountryNames.en = {
    &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in the United States&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Canada&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in the UK&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Ireland&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Australia&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in New Zealand&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Argentina&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Austria&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Belgium&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Brazil&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Switzerland&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Chile&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Colombia&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in the Czech Republic&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Denmark&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Spain&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in France&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Greece&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;HK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Hong Kong&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Hungary&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Israel&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in India&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Italy&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Japan&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Luxembourg&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Latvia&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Mexico&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Malaysia&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in the Netherlands&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Norway&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Peru&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Poland&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Portugal&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Romania&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Sweden&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Slovakia&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Ukraine&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Uruguay&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in South Africa&quot; , &quot;'&quot; , &quot;
};

frb.dayNames = {
    &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;Sunday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Monday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Tuesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Wednesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Thursday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Friday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Saturday&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;ca&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;diumenge&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dilluns&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dimarts&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dimecres&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dijous&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;divendres&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dissabte&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;ja&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;月&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;火&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;水&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;木&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;金&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;土&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;es&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;domingo&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lunes&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;martes&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;miércoles&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;jueves&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;viernes&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sábado&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;sv&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;söndag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;måndag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tisdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;onsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;torsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;fredag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lördag&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;da&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;søndag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mandag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tirsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;onsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;torsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;fredag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lørdag&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;nb&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;søndagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mandagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tirsdagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;onsdagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;torsdagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;fredagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lørdagen&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;it&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;domenica&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lunedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;martedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mercoledì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;giovedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;venerdì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sabato&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;nl&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;zondag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;maandag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dinsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;woensdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;donderdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;vrijdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;zaterdag&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;fr&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;dimanche&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lundi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mardi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mercredi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;jeudi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;vendredi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;samedi&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;de&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;Sonntag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Montag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Dienstag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Mittwoch&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Donnerstag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Freitag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Samstag&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;he&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;ראשון&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;שני&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;שלישי&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;רביעי&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;חמישי&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;שישי&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;שבת&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;lv&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;svētdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;pirmdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;otrdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;trešdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ceturtdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;piektdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sestdienā&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;pl&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;niedzielę&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;poniedziałek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;wtorek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;środę&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;czwartek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;piątek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sobotę&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;neste domingo&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nesta segunda-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nesta terça-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nesta quarta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nesta  quinta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nesta sexta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;neste sábado&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;воскресенье&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;понедельник&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;вторник&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;среду&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;четверг&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;пятницу&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;субботу&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;uk&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;неділі&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;понеділка&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;вівторка&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;середи&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;четверга&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;п’ятниц&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;суботи&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;hu&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;vasárnap&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;hétfő&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;kedd&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;szerda&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;csütörtök&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;péntek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;szombat&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;duminică&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;luni&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;marți&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;miercuri&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;joi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;vineri&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sâmbătă&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;af&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;Sondag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Maandag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Dinsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Woensdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Donderdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Vrydag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Saterdag&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;aa&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;Sunday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Monday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Tuesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Wednesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Thursday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Friday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Saturday&quot; , &quot;'&quot; , &quot; ]
};

// &quot;This fooday&quot; translations. Needed for some languages where gender varies and &quot;this&quot; must agree
frb.dayNamesThis = {
    &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;this Sunday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Monday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Tuesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Wednesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Thursday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Friday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Saturday&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;el&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;Αυτήν την	Κυριακή&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτή τη Δευτέρα&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτήν την	Τρίτη&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτήν την Τετάρτη&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτήν την	Πέμπτη&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτήν την Παρασκευή&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτό το Σάββατο&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;jp&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;この日曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この月曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この火曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この水曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この木曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この金曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この土曜日&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;it&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;questa domenica&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo lunedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo martedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo mercoledì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo giovedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo venerdì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo sabato&quot; , &quot;'&quot; , &quot;],
    &quot; , &quot;'&quot; , &quot;pl&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;w tę niedzielę&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w ten poniedziałek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w ten wtorek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w tę środę&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w ten czwartek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w ten piątek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w tę sobotę&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;в это воскресенье&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в этот понедельник&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в этот вторник&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в эту среду&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в этот четверг&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в эту пятницу&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в эту субботу&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;uk&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;цієї неділі&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цього понеділка&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цього вівторка&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цієї середи&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цього четверга&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цієї п’ятниці&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цієї суботи&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;este domingo&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;esta segunda-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;esta terça-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;esta quarta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;esta quinta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;esta sexta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;este sábado&quot; , &quot;'&quot; , &quot;],
    &quot; , &quot;'&quot; , &quot;sk&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;túto nedeľu&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento pondelok&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento utorok&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;túto stredu&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento štvrtok&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento piatok&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;túto sobotu&quot; , &quot;'&quot; , &quot;],
    &quot; , &quot;'&quot; , &quot;cs&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;tuto neděli&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;toto pondělí&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;toto úterý&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tuto středu&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento čtvrtek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento pátek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tuto sobotu&quot; , &quot;'&quot; , &quot;]
};

frb.iPadTranslations = {
    &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;iPad&quot; , &quot;'&quot; , &quot;
};

// Insert any localize data overrides here

/* jshint maxerr: 600 */
/* MediaWiki:FundraisingBanners/CoreJS-2018.js
 * Core code for banner forms, with new inline error messages
 */

var frb = frb || {};

/**
 * Test for general ES6 support
 *
 * Checks for arrow functions, default parameters, NodeList.prototype.forEach()
 * Should be roughly Chrome 51+, Firefox 50+, Edge 16+, Safari 10+
 * Based on https://gist.github.com/bendc/d7f3dbc83d0f65ca0433caf90378cd95
 * @return {boolean}
 */
frb.supportedBrowser = function() {
    try {
        new Function(&quot; , &quot;'&quot; , &quot;(a = 0) => a&quot; , &quot;'&quot; , &quot;);
        document.querySelectorAll(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;).forEach(a => a);
        return true;
    }
    catch (err) {
        return false;
    }
}();

if ( !mw.centralNotice.adminUi ) { // T262693
    frb.loadedTime = Date.now();
    frb.didSelectAmount = false;
    frb.optinRequiredCountries =
        [ &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot;,
          &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot; ];
    frb.optinRequired = frb.optinRequiredCountries.indexOf(mw.centralNotice.data.country) !== -1;
    frb.maxUSD = 25000;
    frb.reduceMotion = window.matchMedia(&quot; , &quot;'&quot; , &quot;(prefers-reduced-motion: reduce)&quot; , &quot;'&quot; , &quot;).matches;
}

// Keyboard shortcut to go from banner preview to editor - Ctrl+Shift+E
if ( mw.config.get(&quot; , &quot;'&quot; , &quot;wgUserName&quot; , &quot;'&quot; , &quot;) ) {
    if ( mw.config.get(&quot; , &quot;'&quot; , &quot;wgUserName&quot; , &quot;'&quot; , &quot;).match(/\(WMF\)/) ) {
        window.addEventListener(&quot; , &quot;'&quot; , &quot;keydown&quot; , &quot;'&quot; , &quot;, function(e) {
            if ( e.ctrlKey &amp;&amp; e.shiftKey &amp;&amp; e.keyCode === 69 ) {
                window.open( &quot; , &quot;'&quot; , &quot;https://meta.wikimedia.org/wiki/Special:CentralNoticeBanners/Edit/&quot; , &quot;'&quot; , &quot; + mw.centralNotice.data.banner );
            }
        });
    }
}

/**
 * Main function to submit to paymentswiki
 *
 * @param  {Object} options
 * - method (required)
 * - submethod (optional)
 * - gateway (optional)
 * - skipValidation (optional boolean, for pp-usd. Not yet implemented.)
 * @param  {Boolean} isEndowment - deprecated, set frb.isEndowment instead
 */
frb.submitForm = function( options, isEndowment ) {

    var uri = new mw.Uri(&quot; , &quot;'&quot; , &quot;https://payments.wikimedia.org/index.php/Special:GatewayChooser&quot; , &quot;'&quot; , &quot;);
    var params = {};

    if ( !frb.validateForm( options ) ) {
        frb.extraData.validateError = 1; // Flag they had an error, even if fixed later
        return false; // Error, bail out of submitting
    }

    // Skip form chooser for Apple Pay / Google Pay
    if ( options.method === &quot; , &quot;'&quot; , &quot;apple&quot; , &quot;'&quot; , &quot; || options.method === &quot; , &quot;'&quot; , &quot;google&quot; , &quot;'&quot; , &quot; ) {
        uri = new mw.Uri(&quot; , &quot;'&quot; , &quot;https://payments.wikimedia.org/index.php/Special:AdyenCheckoutGateway&quot; , &quot;'&quot; , &quot;);
    }

    // Skip form chooser for Venmo
    if ( options.method === &quot; , &quot;'&quot; , &quot;venmo&quot; , &quot;'&quot; , &quot; ) {
        uri = new mw.Uri(&quot; , &quot;'&quot; , &quot;https://payments.wikimedia.org/index.php/Special:BraintreeGateway&quot; , &quot;'&quot; , &quot;);
    }

    // Form selection data
    params.payment_method = options.method;
    if ( options.submethod ) {
        params.payment_submethod = options.submethod;
    }
    if ( options.gateway ) {
        params.gateway = options.gateway;
    }
    if ( options.variant ) {
        params.variant = options.variant;
    }
    params.recurring = frb.getRecurring();

    if ( params.recurring &amp;&amp; params.variant &amp;&amp; params.variant.match( /monthlyConvert/ ) ) {
        // Post-payments monthly convert makes no sense if it&quot; , &quot;'&quot; , &quot;s already recurring
        // Avoid things like T312905
        delete params.variant;
    }

    params.currency = frb.getCurrency(mw.centralNotice.data.country) || &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot;;

    params.uselang = mw.centralNotice.data.uselang || &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;;
    params.country = mw.centralNotice.data.country || &quot; , &quot;'&quot; , &quot;XX&quot; , &quot;'&quot; , &quot;;

    if ( params.uselang === &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot; &amp;&amp; params.country === &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; ) {
        params.uselang = &quot; , &quot;'&quot; , &quot;pt-br&quot; , &quot;'&quot; , &quot;;
    }
    if ( params.uselang === &quot; , &quot;'&quot; , &quot;es&quot; , &quot;'&quot; , &quot; &amp;&amp;
        ( params.country === &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot; || params.country === &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot; ||
          params.country === &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot; || params.country === &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot; ||
          params.country === &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot; || params.country === &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot; ||
          params.country === &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; )
    ) {
        params.uselang = &quot; , &quot;'&quot; , &quot;es-419&quot; , &quot;'&quot; , &quot;;
    }

    // Adyen override. frb.ccAdyenCountries is defined in LocalizeJS-2017.js
    if ( params.payment_method === &quot; , &quot;'&quot; , &quot;cc&quot; , &quot;'&quot; , &quot; &amp;&amp; frb.ccAdyenCountries.indexOf( params.country ) !== -1 ) {
        params.gateway = &quot; , &quot;'&quot; , &quot;adyen&quot; , &quot;'&quot; , &quot;;
    }
    // dLocal override for South Africa
    if ( params.payment_method === &quot; , &quot;'&quot; , &quot;cc&quot; , &quot;'&quot; , &quot; &amp;&amp; params.country === &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; ) {
        params.gateway = &quot; , &quot;'&quot; , &quot;astropay&quot; , &quot;'&quot; , &quot;;
    }

    // Amount
    var amount = frb.getAmount();
    if ( $(&quot; , &quot;'&quot; , &quot;#frb-ptf-checkbox&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;) ) {
        amount = amount + frb.calculateFee(amount);
        frb.extraData.ptf = 1;
    }
    params.amount = amount;

    // Email optin
    if ( frb.optinRequired &amp;&amp; $(&quot; , &quot;'&quot; , &quot;input[name=&quot;opt_in&quot;]&quot; , &quot;'&quot; , &quot;).length > 0 ) {
        var opt_inValue = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;opt_in&quot;]:checked&quot; , &quot;'&quot; , &quot;).val();
        params.opt_in   = opt_inValue; // frb.validateForm() already checked it&quot; , &quot;'&quot; , &quot;s 1 or 0
    }

    // Tracking info
    if ( isEndowment || frb.isEndowment ) {
        params.utm_medium = &quot; , &quot;'&quot; , &quot;endowment&quot; , &quot;'&quot; , &quot;;
        params.appeal = &quot; , &quot;'&quot; , &quot;EndowmentQuote&quot; , &quot;'&quot; , &quot;;
    } else {
        params.utm_medium = &quot; , &quot;'&quot; , &quot;sitenotice&quot; , &quot;'&quot; , &quot;;
    }
    params.utm_campaign = mw.centralNotice.data.campaign || &quot; , &quot;'&quot; , &quot;test&quot; , &quot;'&quot; , &quot;;
    params.utm_source   = frb.buildUtmSource(params);

    frb.extraData.vw = window.innerWidth;
    frb.extraData.vh = window.innerHeight;
    frb.extraData.time = Math.round( (Date.now() - frb.loadedTime)/1000 );

    if ( navigator.brave !== undefined ) { // T283367
        frb.extraData.brave = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
    }

    if ( !$.isEmptyObject( frb.extraData ) ) {
        params.utm_key = frb.buildUtmKey( frb.extraData );
    }

    // Link to Banner History if enabled
    var mixins = mw.centralNotice.getDataProperty( &quot; , &quot;'&quot; , &quot;mixins&quot; , &quot;'&quot; , &quot; );
    if ( mixins &amp;&amp; mixins.bannerHistoryLogger ) {
        params.bannerhistlog = mw.centralNotice.bannerHistoryLogger.id;
    }

    uri.extend(params);

    // Set a cookie with current location so we can return here from TY page
    mw.loader.using( [ &quot; , &quot;'&quot; , &quot;mediawiki.cookie&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mediawiki.util&quot; , &quot;'&quot; , &quot; ] ).then( function () {
        // Exclude URL parameters like banner, but cope with paths like /w/index.php?title=Foo
        var returnToUrl = window.location.origin + mw.util.getUrl();
        mw.cookie.set(
            &quot; , &quot;'&quot; , &quot;fundraising_returnTo&quot; , &quot;'&quot; , &quot;,
            returnToUrl,
            { expires: 300, prefix: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, domain: &quot; , &quot;'&quot; , &quot;.wikipedia.org&quot; , &quot;'&quot; , &quot;, secure: true }
        );
    });

    if ( mixins &amp;&amp; mixins.bannerHistoryLogger ) {
        mw.centralNotice.bannerHistoryLogger.ensureLogSent().always(function() {
            frb.goToPayments( uri );
        });
    } else {
        frb.goToPayments( uri );
    }

};

frb.goToPayments = function( uri ) {
    if ( window.top !== window.self ) {
        // banner is in a frame, open payments in a new tab
        window.open( uri.toString() );
    } else {
        window.location.href = uri.toString();
    }
};

/**
 * Check the form for errors.
 *
 * Called on submission, can also be called on input
 *
 * @param {object} options
 * @return {boolean} Whether form is error-free
 */
frb.validateForm = function( options ) {
    var error = false;

    /* Reset all errors */
    $(&quot; , &quot;'&quot; , &quot;.frb-haserror&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
    $(&quot; , &quot;'&quot; , &quot;.frb-error&quot; , &quot;'&quot; , &quot;).hide();

    if ( !options.method ) {
        error = true;
        $(&quot; , &quot;'&quot; , &quot;.frb-methods&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-method&quot; , &quot;'&quot; , &quot;).show();
    }

    if ( !frb.validateAmount() ) {
        error = true;
    }

    /* Email optin */
    if ( frb.optinRequired &amp;&amp; $(&quot; , &quot;'&quot; , &quot;.frb-optin&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:visible&quot; , &quot;'&quot; , &quot;) ) {
        var opt_inValue = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;opt_in&quot;]:checked&quot; , &quot;'&quot; , &quot;).val();
        if ( opt_inValue !== &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot; &amp;&amp; opt_inValue !== &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot; ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-optin&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-error-optin&quot; , &quot;'&quot; , &quot;).show();
            error = true;
        }
    }

    return !error;
};

/**
 * Check if selected amount is valid i.e. a positive number, between minimum and maximum.
 * If not, show an error and return false.
 */
frb.validateAmount = function() {

    var amount = frb.getAmount(),
        currency  = frb.getCurrency( mw.centralNotice.data.country ),
        minAmount = frb.amounts.minimums[ currency ],
        maxAmount = Math.round( frb.maxUSD * minAmount );
        // Math.round to account for floating point math errors: https://phabricator.wikimedia.org/T246262

    if ( amount === null || isNaN(amount) || amount &lt;= 0 || amount &lt; minAmount ) {
        $(&quot; , &quot;'&quot; , &quot;fieldset.frb-amounts&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-bigamount&quot; , &quot;'&quot; , &quot;).hide();
        $(&quot; , &quot;'&quot; , &quot;.frb-error-smallamount&quot; , &quot;'&quot; , &quot;).show();
        return false;
    } else if ( amount > Math.round( maxAmount ) ) {
        $(&quot; , &quot;'&quot; , &quot;fieldset.frb-amounts&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-bigamount&quot; , &quot;'&quot; , &quot;).show();
        return false;
    } else {
        $(&quot; , &quot;'&quot; , &quot;fieldset.frb-amounts&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-smallamount, .frb-error-bigamount&quot; , &quot;'&quot; , &quot;).hide();
        return true;
    }
};

/**
 * Build the utm_source for analytics.
 *
 * Own function so it can be overriden for weird tests
 *
 * @param  {Object} params
 * @return {string} utm_source
 */
frb.buildUtmSource = function(params) {

    var utm_source;
    var fullDottedPaymentMethod = params.payment_method;
    if ( params.recurring ) {
        fullDottedPaymentMethod = &quot; , &quot;'&quot; , &quot;r&quot; , &quot;'&quot; , &quot; + fullDottedPaymentMethod;
    }
    if ( params.payment_submethod ) {
        fullDottedPaymentMethod = fullDottedPaymentMethod + &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; + params.payment_submethod;
    }

    utm_source = mw.centralNotice.data.banner;

    // Keeping opt-in in utm_source for safety for now
    // Eventually remove it, or move to utm_key?
    if ( params.opt_in ) {
        utm_source += &quot; , &quot;'&quot; , &quot;_optIn&quot; , &quot;'&quot; , &quot; + params.opt_in;
    }

    utm_source += &quot; , &quot;'&quot; , &quot;.no-LP.&quot; , &quot;'&quot; , &quot; + fullDottedPaymentMethod;

    return utm_source;
};

/**
 * Build a string for utm_key from extra tracking data
 *
 * @param  {Object} data
 * @return {string} utm_key
 */
frb.buildUtmKey = function(data) {
    var dataArray = [];
    for (var key in data) {
        if (data.hasOwnProperty(key)) {
            dataArray.push( key + &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; + data[key] );
        }
    }
    return dataArray.join(&quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot;);
};

/**
 * Determine if we should show recurring choice on step 2
 * 
 * NOTE 2023-12-07: we don&quot; , &quot;'&quot; , &quot;t currently use this for step 2, since there are no
 *	banners where users select method before frequency. However it is used by
 *	frb.shouldShowMonthlyConvert()
 *
 * @param  {Object} options     Including method and optional gateway
 * @param  {String} country
 * @return {boolean}
 */
frb.shouldShowRecurring = function( options, country ) {

    if ( frb.isEndowment ) {
        return false;
    }
    if ( frb.noRecurringCountries.indexOf( country ) !== -1 ) { // Defined in LocalizeJS-2017.js
        return false;
    }
    if ( options.method === undefined ) {
        return true; // Show if a method hasn&quot; , &quot;'&quot; , &quot;t been selected yet
    }
    if ( [ &quot; , &quot;'&quot; , &quot;cc&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;paypal&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;venmo&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;apple&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;google&quot; , &quot;'&quot; , &quot; ].indexOf( options.method ) !== -1 ) {
        return true;
    }
    // Adyen iDEAL
    if ( options.submethod === &quot; , &quot;'&quot; , &quot;rtbt_ideal&quot; , &quot;'&quot; , &quot; ) {
        return true;
    }
    if ( options.submethod === &quot; , &quot;'&quot; , &quot;upi&quot; , &quot;'&quot; , &quot; || options.submethod === &quot; , &quot;'&quot; , &quot;paytmwallet&quot; , &quot;'&quot; , &quot; ) {
        return true;
    }
    return false;
};

/* Is recurring method selected? This function can be overriden for different forms */
frb.getRecurring = function() {
    // Can&quot; , &quot;'&quot; , &quot;t use simple form.frequency.value, doesn&quot; , &quot;'&quot; , &quot;t work in IE
    var selected = $(&quot; , &quot;'&quot; , &quot;#frb-form input[name=&quot;frequency&quot;]:checked&quot; , &quot;'&quot; , &quot;).val();
    return selected === &quot; , &quot;'&quot; , &quot;monthly&quot; , &quot;'&quot; , &quot;;
};

/* Return amount selected */
frb.getAmount = function() {
    var form = document.getElementById(&quot; , &quot;'&quot; , &quot;frb-form&quot; , &quot;'&quot; , &quot;);
    var amount = null;
    frb.extraData.otherAmt = 0;

    // If there are some amount radio buttons, then look for the checked one
    if (form.amount) {
        for (var i = 0; i &lt; form.amount.length; i++) {
            if (form.amount[i].checked) {
                amount = form.amount[i].value;
            }
        }
    }

    // Check the &quot;other&quot; amount box
    if (form.otherAmount.value !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
        var otherAmount = form.otherAmount.value;
        otherAmount = otherAmount.replace(/[,.](\d)$/, &quot; , &quot;'&quot; , &quot;:$10&quot; , &quot;'&quot; , &quot;);
        otherAmount = otherAmount.replace(/[,.](\d)(\d)$/, &quot; , &quot;'&quot; , &quot;:$1$2&quot; , &quot;'&quot; , &quot;);
        otherAmount = otherAmount.replace(/[$£€¥,.]/g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        otherAmount = otherAmount.replace(/:/, &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;);
        amount = otherAmount;
        frb.extraData.otherAmt = 1;
    }

    amount = parseFloat(amount);

    if ( isNaN(amount) ) {
        return 0;
    } else {
        return amount;
    }

};

/* Localize the amount errors. Call when initialising banner. */
frb.localizeErrors = function() {
    var currency  = frb.getCurrency( mw.centralNotice.data.country ),
        language = mw.centralNotice.data.uselang,
        minAmount = frb.amounts.minimums[ currency ],
        maxAmount = Math.round( frb.maxUSD * minAmount );
        // Math.round to account for floating point math errors: https://phabricator.wikimedia.org/T246262

    $(&quot; , &quot;'&quot; , &quot;.frb-error-smallamount&quot; , &quot;'&quot; , &quot;).text( function( index, oldText ) {
        return oldText.replace( &quot; , &quot;'&quot; , &quot;$1&quot; , &quot;'&quot; , &quot;, frb.formatCurrency(currency, minAmount, language)  );
    });

    $(&quot; , &quot;'&quot; , &quot;.frb-error-bigamount&quot; , &quot;'&quot; , &quot;).text( function( index, oldText ) {
        // We cannot accept donations greater than $1 $2 through our website. Please contact our major gifts staff at $3.
        return oldText.replace( &quot; , &quot;'&quot; , &quot;$1&quot; , &quot;'&quot; , &quot;, maxAmount )
                      .replace( &quot; , &quot;'&quot; , &quot;$2&quot; , &quot;'&quot; , &quot;, currency )
                      .replace( &quot; , &quot;'&quot; , &quot;$3&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;benefactors@wikimedia.org&quot; , &quot;'&quot; , &quot; );
    });
};

/**
 * Shared code for amount input handling
 */
frb.initAmountOptions = function() {

    // Reset &quot;Other&quot; input if user clicks a preset amount
    $(&quot; , &quot;'&quot; , &quot;#frb-form [id^=frb-amt-ps]&quot; , &quot;'&quot; , &quot;).click(function() {
        $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    });

    // Track if they selected and then later changed amount
    var checkAmountChange = function(e) {
        if ( frb.didSelectAmount ) {
            frb.extraData.changedAmt = 1;
        }
        // check if amount radio button is selected OR there is a value in the other amount
        if ( $(&quot; , &quot;'&quot; , &quot;.frb-amounts input[type=&quot;radio&quot;]:checked&quot; , &quot;'&quot; , &quot;).val() !== &quot; , &quot;'&quot; , &quot;Other&quot; , &quot;'&quot; , &quot; || $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).val().length > 0 ) {
            frb.didSelectAmount = true;
        }
        return;
    };

    $(&quot; , &quot;'&quot; , &quot;.frb-amounts input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, checkAmountChange);
    $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focusout&quot; , &quot;'&quot; , &quot;, checkAmountChange);

    // Block typing non-numerics in input field, otherwise Safari allows them and then chokes
    // https://phabricator.wikimedia.org/T118741, https://phabricator.wikimedia.org/T173431
    var blockNonNumeric = function(e) {
        // Allow special keys in Firefox
        if ((e.code == &quot; , &quot;'&quot; , &quot;ArrowLeft&quot; , &quot;'&quot; , &quot;) || (e.code == &quot; , &quot;'&quot; , &quot;ArrowRight&quot; , &quot;'&quot; , &quot;) ||
            (e.code == &quot; , &quot;'&quot; , &quot;ArrowUp&quot; , &quot;'&quot; , &quot;) || (e.code == &quot; , &quot;'&quot; , &quot;ArrowDown&quot; , &quot;'&quot; , &quot;) ||
            (e.code == &quot; , &quot;'&quot; , &quot;Delete&quot; , &quot;'&quot; , &quot;) || (e.code == &quot; , &quot;'&quot; , &quot;Backspace&quot; , &quot;'&quot; , &quot;)) {
            return;
        }
        var chr = String.fromCharCode(e.which);
        if (&quot;0123456789., &quot;.indexOf(chr) === -1) {
            return false;
        }
    };
    $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;keypress&quot; , &quot;'&quot; , &quot;, blockNonNumeric);
    $(&quot; , &quot;'&quot; , &quot;#frb-amt-monthly-other-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;keypress&quot; , &quot;'&quot; , &quot;, blockNonNumeric);

};

/**
 * Calculate approximate transaction fee on given amount
 *
 * @param  {number} amount
 * @return {number}        Rounded to 2 decimal places
 */
frb.calculateFee = function(amount) {
    var currency = frb.getCurrency(mw.centralNotice.data.country),
        feeMultiplier = 0.04,
        feeMinimum = frb.amounts.feeMinimums[currency] || 0.35,
        feeAmount = amount * feeMultiplier;

    if ( feeAmount &lt; feeMinimum ) {
      feeAmount = feeMinimum;
    }
    return parseFloat(feeAmount.toFixed(2));
};

frb.updateFeeDisplay = function() {
    var currency = frb.getCurrency(mw.centralNotice.data.country),
        language = mw.centralNotice.data.uselang,
        amount, feeAmount, totalAmount;

    amount = frb.getAmount();
    feeAmount = frb.calculateFee(amount);
    if ( $(&quot; , &quot;'&quot; , &quot;#frb-ptf-checkbox&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;) ) {
        totalAmount = amount + feeAmount;
    } else {
        totalAmount = amount;
    }

    var feeAmountFormatted = frb.formatCurrency(currency, feeAmount, language);
    $(&quot; , &quot;'&quot; , &quot;.frb-ptf-fee&quot; , &quot;'&quot; , &quot;).text(feeAmountFormatted);

    var totalAmountFormatted = frb.formatCurrency(currency, totalAmount, language);
    $(&quot; , &quot;'&quot; , &quot;.frb-ptf-total&quot; , &quot;'&quot; , &quot;).text(totalAmountFormatted);

    $(&quot; , &quot;'&quot; , &quot;.frb-ptf&quot; , &quot;'&quot; , &quot;).slideDown( frb.reduceMotion ? 0 : 400 );
};

/**
 * Custom hide cookie function
 *
 * Purposely sets only for this domain.
 * CentralNotice builtin method seems buggy - see T270401
 *
 * @param {string} reason Reason to store in the hide cookie
 * @param {number} duration Cookie duration, in seconds
 */
frb.altSetHideCookie = function ( reason, duration ) {

    mw.loader.using( &quot; , &quot;'&quot; , &quot;mediawiki.cookie&quot; , &quot;'&quot; , &quot; ).then( function () {

        var cookieName = &quot; , &quot;'&quot; , &quot;centralnotice_hide_fundraising&quot; , &quot;'&quot; , &quot;,
            date = new Date(),
            hideData = {
                v: 1,
                created: Math.floor( date.getTime() / 1000 ),
                reason: reason
            };

        // Re-use the same date object to set the cookie&quot; , &quot;'&quot; , &quot;s expiry time
        date.setSeconds( date.getSeconds() + duration );

        mw.cookie.set(
            cookieName,
            JSON.stringify( hideData ),
            { expires: date, path: &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;, domain: &quot; , &quot;'&quot; , &quot;wikipedia.org&quot; , &quot;'&quot; , &quot;, prefix: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; }
        );

    });

};

/**
 * Determine if banner should be shown, and set correct data for impression logging
 *
 * @return {boolean} Show banner?
 */
frb.shouldShowBanner = function() {

    mw.centralNotice.bannerData.hideResult = false;

    /* Hide in unsupported browsers */
    if ( !frb.supportedBrowser ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = &quot; , &quot;'&quot; , &quot;browser&quot; , &quot;'&quot; , &quot;;
    }

    /* Hide outside main namespace (except Main Page, for sites where it isn&quot; , &quot;'&quot; , &quot;t in main namespace) */
    if ( mw.config.get(&quot; , &quot;'&quot; , &quot;wgNamespaceNumber&quot; , &quot;'&quot; , &quot;) > 0 &amp;&amp; !mw.config.get(&quot; , &quot;'&quot; , &quot;wgIsMainPage&quot; , &quot;'&quot; , &quot;) ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = &quot; , &quot;'&quot; , &quot;namespace&quot; , &quot;'&quot; , &quot;;
    }

    // Hide banner on sensitive articles
    // TODO - possibly add wgWikibaseItemId for multilingual support and resilience to moves?
    var hideTitles = [ &quot; , &quot;'&quot; , &quot;Murder of Don Banfield&quot; , &quot;'&quot; , &quot; ];
    if ( hideTitles.indexOf( mw.config.values.wgTitle ) !== -1 ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = &quot; , &quot;'&quot; , &quot;article&quot; , &quot;'&quot; , &quot;;
    }

    /* Hide banner if on wrong site (desktop/mobile) in case wrong device settings were chosen */
    var bannerName = mw.centralNotice.data.banner,
        skin = mw.config.get(&quot; , &quot;'&quot; , &quot;skin&quot; , &quot;'&quot; , &quot;);
    if (
         ( bannerName.indexOf(&quot; , &quot;'&quot; , &quot;_dsk_&quot; , &quot;'&quot; , &quot;) !== -1 &amp;&amp; skin === &quot; , &quot;'&quot; , &quot;minerva&quot; , &quot;'&quot; , &quot; ) ||
         ( bannerName.indexOf(&quot; , &quot;'&quot; , &quot;_m_&quot; , &quot;'&quot; , &quot;) !== -1 &amp;&amp; skin !== &quot; , &quot;'&quot; , &quot;minerva&quot; , &quot;'&quot; , &quot; )
    ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = &quot; , &quot;'&quot; , &quot;other&quot; , &quot;'&quot; , &quot;;
        console.warn(&quot; , &quot;'&quot; , &quot;Hiding fundraising banner on wrong site (desktop/mobile)&quot; , &quot;'&quot; , &quot;);
    }

    return !mw.centralNotice.bannerData.hideResult;

};

/* Debug function to highlight dynamically replaced elements */
frb.highlightReplacements = function() {
    $(&quot; , &quot;'&quot; , &quot;.frb [class^=&quot;frb-replace&quot;], .frb-ptf-fee, .frb-ptf-total, .frb-upsell-ask, frb-amt&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;background-color&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#fa0&quot; , &quot;'&quot; , &quot;);
};

if ( !mw.centralNotice.adminUi ) { // T262693
    /**
     * Provides alterImpressionData hook for CentralNotice
     * This info will be sent back with Special:RecordImpression
     * TODO: check if/when we can remove this (and RecordImpression)
     */
    mediaWiki.centralNotice.bannerData.alterImpressionData = function( impressionData ) {
        // Returning true from this function indicates the banner was shown
        if (mediaWiki.centralNotice.bannerData.hideReason) {
            impressionData.reason = mediaWiki.centralNotice.bannerData.hideReason;
        }
        if (mediaWiki.centralNotice.bannerData.cookieCount) {
            impressionData.banner_count = mediaWiki.centralNotice.bannerData.cookieCount;
        }

        return !mediaWiki.centralNotice.bannerData.hideResult;
    };
}

/* End of MediaWiki:FundraisingBanners/CoreJS-2018.js */
/* jshint maxerr: 600 */
/* == MediaWiki:FundraisingBanners/LocalizeJS-2022.js == */

/**
 * Get the currency for a given country
 *
 * NOTE: The following currency mapping is WMF-specific based on payment
 * provider availability, NOT necessarily the official currency of the country
 *
 * @param  {string} country code
 * @return {string} currency code
 */
frb.getCurrency = function(country) {
    switch ( country ) {
        // Big 6
        case &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot;;
        // Euro countries
        case &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot;:
            return &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot;;
        // Others
        case &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot;;
        // Latin America
        case &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;LI&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot;;
        // Fall back to USD
        default:
            return &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot;;
    }
};

/**
 * Format a currency value
 *
 * TODO: make this handle the ISO code overrides?
 *
 * @param  {string} currency code. Leave undefined to get without symbol.
 * @param  {number} amount
 * @param  {string} language code
 * @return {string} formatted string e.g. &quot; , &quot;'&quot; , &quot;$3&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;£5&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;10 €&quot; , &quot;'&quot; , &quot;
 */
frb.formatCurrency = function(currency, amount, language) {

    var locale, formatterOptions, formatter, fmAmount, supportsIntl;

    if ( isNaN(amount) || amount === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ) {
        // Not a number, it&quot; , &quot;'&quot; , &quot;s probably the &quot; , &quot;'&quot; , &quot;other&quot; , &quot;'&quot; , &quot; string or box
        // TODO: better way of doing this?
        fmAmount = amount;
    } else {
        // Check browser support
        try {
            supportsIntl = typeof window.Intl === &quot; , &quot;'&quot; , &quot;object&quot; , &quot;'&quot; , &quot;;
        } catch (e) {
            supportsIntl = false; // T265396
        }

        if ( supportsIntl ) {
            // Use Intl for fancy number formatting - thousands separators etc
            locale = language + &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; + mw.centralNotice.data.country;
            if ( amount % 1 !== 0 ) {
                formatterOptions = { minimumFractionDigits: 2 };
            } else {
                formatterOptions = {};
            }
            formatter = new Intl.NumberFormat(locale, formatterOptions);
        } else {
            // Bad browser i.e. IE. Just do the basics: 2 decimal places if needed, or none
            formatter = {};
            formatter.format = function(number) {
                if ( amount % 1 !== 0 ) {
                    return number.toFixed(2);
                } else {
                    return number.toString();
                }
            };
        }
        fmAmount = formatter.format(amount);
    }

    // No symbol needed
    if ( currency === undefined ) {
        return fmAmount;
    }

    // Better dive into the formatting object
    if ( frb.currencyFormats[currency] === undefined ) {
        return currency + &quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot; + fmAmount;
    }
    if ( frb.currencyFormats[currency] instanceof Object ) { // not a string
        if ( frb.currencyFormats[currency][language] !== undefined ) {
            return frb.currencyFormats[currency][language].replace(&quot; , &quot;'&quot; , &quot;\t&quot; , &quot;'&quot; , &quot;, fmAmount);
        }
        return frb.currencyFormats[currency][&quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot;].replace(&quot; , &quot;'&quot; , &quot;\t&quot; , &quot;'&quot; , &quot;, fmAmount);
    }

    return frb.currencyFormats[currency].replace(&quot; , &quot;'&quot; , &quot;\t&quot; , &quot;'&quot; , &quot;, fmAmount);
};

/*
 * Select the correct amount or array of amounts from object in &quot;source&quot;
 *
 * @param {Object} source   - the amounts data object e.g. frb.amounts.options7, frb.amounts.averages
 * @param {string} currency - ISO code of currency
 * @param {string} country  - ISO code of country (optional)
 * @return {array/number}   - depending on source
 */
frb.pickAmounts = function(source, currency, country) {

    if ( source[currency][&quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot;] ) { // we need to go deeper
        if ( source[currency][country] !== undefined ) {
            return source[currency][country];
        } else {
            return source[currency][&quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot;];
        }
    } else {
        return source[currency];
    }
};

/* Credit card types so we can show the correct logos */
frb.cardTypes = {
    // Big 6
    &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmad&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    // Euro countries
    &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;, // Adyen
    &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;, // Adyen
    &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    // Others
    &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmad&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmad&quot; , &quot;'&quot; , &quot;, // Adyen
    &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;, // Adyen
    &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vm&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmar&quot; , &quot;'&quot; , &quot;, // dLocal
    &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;LI&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;
};

/**
 * Should we show Apple Pay?
 *
 * Note there is a ~500ms delay in Safari when checking, so only call this if needed
 *
 * @param  {string} country
 * @return {boolean}
 */
frb.shouldShowApplePay = function ( country ) {
    // https://support.apple.com/en-us/HT207957 - minus China mainland
    var appleCountries = [
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TW&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;EE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FI&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;IS&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;JE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LI&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;MT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MC&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ME&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SM&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;RS&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SI&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VA&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;BH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PS&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;QA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AE&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;
    ];
    if ( appleCountries.indexOf( country ) === -1 ) {
        return false;
    }
    if ( location.search.match(&quot; , &quot;'&quot; , &quot;forceApplePay&quot; , &quot;'&quot; , &quot;) ) {
        return true;
    }
    if ( window.ApplePaySession ) {
        if ( ApplePaySession.canMakePayments() ) {
            return true;
        }
    }
    return false;
};

/**
 * Display the correct payment methods for current country
 *
 * Methods should be labeled with class &quot; , &quot;'&quot; , &quot;frb-pm-xxxx&quot; , &quot;'&quot; , &quot;
 * TODO: clean this function up more
 *
 * @param  {string} country
 */
frb.localizeMethods = function(country) {

    // Test country with *all the methods*
    if ( country === &quot; , &quot;'&quot; , &quot;ZZ&quot; , &quot;'&quot; , &quot; ) {
        $(&quot; , &quot;'&quot; , &quot;.frb-payment-options > div&quot; , &quot;'&quot; , &quot;).show();
        return;
    }

    // Hide recurring completely for some countries and endowment
    if ( frb.isEndowment || frb.noRecurringCountries.indexOf(country) !== -1 ) {
        $(&quot; , &quot;'&quot; , &quot;.frb-frequency, .recurring-details&quot; , &quot;'&quot; , &quot;).hide();
    }

    // Remove any leftover WorldPay and Adyen
    $(&quot; , &quot;'&quot; , &quot;.frb-pm-cc-wp&quot; , &quot;'&quot; , &quot;).remove();
    $(&quot; , &quot;'&quot; , &quot;.frb-pm-cc-adyen&quot; , &quot;'&quot; , &quot;).remove();

    // Monthly Adyen credit card is allowed now
    // if ( frb.ccAdyenCountries.indexOf( country ) !== -1 ) {
    //     $(&quot; , &quot;'&quot; , &quot;.frb-pm-cc&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;no-monthly&quot; , &quot;'&quot; , &quot;);
    // }

    // Countries with no PayPal option
    var noPP = [&quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;QA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;OM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BD&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PA&quot; , &quot;'&quot; , &quot;,
                &quot; , &quot;'&quot; , &quot;PY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;JM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DZ&quot; , &quot;'&quot; , &quot;];
    if ($.inArray(country, noPP) !== -1) {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp&quot; , &quot;'&quot; , &quot;).remove();
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp-usd&quot; , &quot;'&quot; , &quot;).remove();
    }

    // Countries with no PayPal for mobile only - https://phabricator.wikimedia.org/T173001
    var noPPmobile = [&quot; , &quot;'&quot; , &quot;PH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ID&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VN&quot; , &quot;'&quot; , &quot;];
    var mobileRegex = /(_mob_|_ipd_|_m_)/;
    if ($.inArray(country, noPPmobile) !== -1) {
        if (mw.centralNotice.data.banner.search(mobileRegex) !== -1) {
            $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp&quot; , &quot;'&quot; , &quot;).remove();
            $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp-usd&quot; , &quot;'&quot; , &quot;).remove();
        }
    }

    // Countries where PayPal must be in USD
    var ppUSD = [&quot; , &quot;'&quot; , &quot;BG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ID&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KR&quot; , &quot;'&quot; , &quot;,
                 &quot; , &quot;'&quot; , &quot;KZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NI&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot;,
                 &quot; , &quot;'&quot; , &quot;BH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IS&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MV&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BZ&quot; , &quot;'&quot; , &quot;,
                 &quot; , &quot;'&quot; , &quot;CR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CW&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LC&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GD&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FJ&quot; , &quot;'&quot; , &quot;,
                 &quot; , &quot;'&quot; , &quot;TN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BJ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BF&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CI&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GW&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ML&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;,
                 &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;];
    if ($.inArray(country, ppUSD) !== -1) {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp&quot; , &quot;'&quot; , &quot;).remove();
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp-usd&quot; , &quot;'&quot; , &quot;).show();
    } else {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp-usd&quot; , &quot;'&quot; , &quot;).remove();
    }

    // Show any extra local payment methods, or remove them if not needed
    var extrapaymentmethods = {
        &quot; , &quot;'&quot; , &quot;amazon&quot; , &quot;'&quot; , &quot;   : [&quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;], // Note Amazon was removed from current best 2023-10-20
        &quot; , &quot;'&quot; , &quot;bpay&quot; , &quot;'&quot; , &quot;     : [],
        &quot; , &quot;'&quot; , &quot;ideal&quot; , &quot;'&quot; , &quot;    : [&quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;],
        &quot; , &quot;'&quot; , &quot;bt&quot; , &quot;'&quot; , &quot;       : [&quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;], // Bank Transfer (dLocal/Adyen)
        &quot; , &quot;'&quot; , &quot;cash&quot; , &quot;'&quot; , &quot;     : [&quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot;],  // &quot; , &quot;'&quot; , &quot;Cash&quot; , &quot;'&quot; , &quot; methods (dLocal)
        &quot; , &quot;'&quot; , &quot;pix&quot; , &quot;'&quot; , &quot;      : [&quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;],
        &quot; , &quot;'&quot; , &quot;boleto&quot; , &quot;'&quot; , &quot;   : [&quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;]
    };

    // Methods with different labels per country

    var language = mw.config.get(&quot; , &quot;'&quot; , &quot;wgUserLanguage&quot; , &quot;'&quot; , &quot;);
    var btTranslation = &quot; , &quot;'&quot; , &quot;Bank Transfer&quot; , &quot;'&quot; , &quot;;

    if (language === &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot;) {

        if (country === &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;) {
            btTranslation = &quot; , &quot;'&quot; , &quot;Transferência bancária&quot; , &quot;'&quot; , &quot;;
        }

    } else if (language === &quot; , &quot;'&quot; , &quot;es&quot; , &quot;'&quot; , &quot;) {

        if (country === &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;) {
            btTranslation = &quot; , &quot;'&quot; , &quot;WebPay&quot; , &quot;'&quot; , &quot;;
        } else if (country === &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;) {
            btTranslation = &quot; , &quot;'&quot; , &quot;PSE Pagos&quot; , &quot;'&quot; , &quot;;
        } else {
            btTranslation = &quot; , &quot;'&quot; , &quot;Transferencia bancaria&quot; , &quot;'&quot; , &quot;;
        }

    }

    if (country === &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;) {
        if (language === &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;) {
            btTranslation = &quot; , &quot;'&quot; , &quot;Online Banking&quot; , &quot;'&quot; , &quot;;
        }
        if (language === &quot; , &quot;'&quot; , &quot;cs&quot; , &quot;'&quot; , &quot;) {
            btTranslation = &quot; , &quot;'&quot; , &quot;Internetové Bankovnictví&quot; , &quot;'&quot; , &quot;;
        }
    }

    $( &quot; , &quot;'&quot; , &quot;.frb-pm-bt button, .frb-pm-bt label, button.frb-pm-bt&quot; , &quot;'&quot; , &quot; ).text( btTranslation );

    for (var method in extrapaymentmethods) {
        var $methodbutton = $(&quot; , &quot;'&quot; , &quot;.frb-pm-&quot; , &quot;'&quot; , &quot; + method);
        if ( $.inArray(country, extrapaymentmethods[method]) !== -1 &amp;&amp; !frb.isEndowment ) {
            $methodbutton.show();
        } else {
            $methodbutton.remove();
        }
    }

    // Google Pay - separated from extrapaymentmethods as we want to show on Endowment too
    var googlePayCountries = [
        &quot; , &quot;'&quot; , &quot;AE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;EE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HR&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;OM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;QA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TW&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot;
    ];
    if ( $.inArray(country, googlePayCountries) !== -1 ) {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-google&quot; , &quot;'&quot; , &quot;).show();
    } else {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-google&quot; , &quot;'&quot; , &quot;).remove();
    }

    // Apple Pay
    if ( $(&quot; , &quot;'&quot; , &quot;.frb-pm-applepay&quot; , &quot;'&quot; , &quot;).length > 0 ) {
        if ( !frb.shouldShowApplePay( country ) ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-pm-applepay&quot; , &quot;'&quot; , &quot;).remove();
        }
    }

	// Venmo
	var $venmo = $(&quot; , &quot;'&quot; , &quot;.frb-pm-venmo&quot; , &quot;'&quot; , &quot;);
	if ( country === &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; &amp;&amp; $venmo.length > 0 ) {
		// From MediaWiki:FundraisingBanners/VenmoBrowserCheck.js
		if ( frb.isVenmoSupported() ) {
			$venmo.show();
		} else {
			$venmo.remove();
		}
	} else {
		$venmo.remove();
	}

    /* Add card types class to credit card button, so we can show correct logos */
    if ( frb.cardTypes[country] ) {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-cc&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-cctypes-&quot; , &quot;'&quot; , &quot; + frb.cardTypes[country] );
    }
};

/**
 * Check scheduled payment method outages and hide buttons if needed
 *
 * Data at https://meta.wikimedia.org/wiki/MediaWiki:FR2013/Resources/PaymentOutages.js
 * Methods should be labeled with class &quot; , &quot;'&quot; , &quot;frb-pm-xxxx&quot; , &quot;'&quot; , &quot;
 *
 * @param  {string} country code
 */
frb.checkMethodOutages = function(country) {

    // TODO - can we load this a better way?
    /* This file can be used to schedule hiding of individual payment methods from banners
 * e.g. if they have scheduled downtime.
 *
 * Valid methods are:
 *	ideal, cc, pp, amazon, bpay, webmoney, cash, pp-usd
 * (most of the time it&quot; , &quot;'&quot; , &quot;s &quot; , &quot;'&quot; , &quot;ideal&quot; , &quot;'&quot; , &quot;...)
 * Can also limit outage to a specific country with country: &quot;XX&quot; (where XX is an ISO code)
 *
 * Note that in JavaScript dates the months (and only the months) start at 0.
 * Jan=0, Feb=1, Mar=2, Apr=3 etc. How hateful.
 *
 * Be sure to also update donatewiki if needed e.g. by commenting the method templates
 * found at https://donate.wikimedia.org/wiki/Template:2012FR/Form-section/Paymentmethods
 * 
 */
var outages = [
    {
        start:      new Date(Date.UTC(2016, 8, 18, 1)),
        end:        new Date(Date.UTC(2016, 8, 18, 7)),
        method:     &quot;ideal&quot;
    }
]; // jshint ignore:line
    var now = new Date();

    for (var i = outages.length - 1; i >= 0; i--) {
        if ( now > outages[i].start &amp;&amp; now &lt; outages[i].end ) {
            if (outages[i].country === undefined || outages[i].country == country) {
                $(&quot; , &quot;'&quot; , &quot;.frb-pm-&quot; , &quot;'&quot; , &quot; + outages[i].method).hide();
            }
        }
    }
};

/**
 * Adjust the amount options and their labels
 *
 * Inputs should have id frb-amt-psX where X is the index number (starting from 1)
 *
 * @param  {Object}  source     - object with amounts e.g. frb.amounts.options7
 * @param  {string}  currency   - currency code e.g. &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot;
 * @param  {string}  country    - country code  e.g. &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot; Some currencies can have different options per country.
 * @param  {string}  language   - language code e.g. &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; For symbol formatting
 * @param  {boolean} useSymbols - use currency symbols on labels or not? (3 vs $3)
 */
frb.localizeAmountOptions = function(source, currency, country, language, useSymbols) {

    var amountOptions = frb.pickAmounts(source, currency, country);

    $(&quot; , &quot;'&quot; , &quot;#frb-form input[name=&quot;amount&quot;]&quot; , &quot;'&quot; , &quot;).each(function(index) {
        var $input = $(this);
        var $label = $input.siblings(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;);

        var i = $input.attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;).replace(&quot; , &quot;'&quot; , &quot;frb-amt-ps&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var amount = amountOptions[i-1]; // because IDs start from 1

        if ( amount ) {
            $input.val( amount );
            if ( useSymbols ) {
                $label.text( frb.formatCurrency( currency, amount, language) );
            } else {
                $label.text( frb.formatCurrency( undefined, amount, language) );
            }
        }
    });

};

/**
 * Make an element into a link
 *
 * @param  {string} selector    CSS selector for elements to convert to a link
 * @param  {string} language    Code of language (could be es-419 or pt-br)
 * @param  {string} baseUrl     URL of link (function will add language parameter)
 */
frb.makeLink = function( selector, language, baseUrl ) {
    var url = baseUrl + &quot; , &quot;'&quot; , &quot;&amp;language=&quot; , &quot;'&quot; , &quot; + language;
    $( selector ).each( function() {
        var $link = $( &quot; , &quot;'&quot; , &quot;&lt;a>&lt;/a>&quot; , &quot;'&quot; , &quot; );
        $link.html( $( this ).html() );
        $link.attr( { href: url, target: &quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot; } );
        $( this ).replaceWith( $link );
    });
};

/**
 * Get the number of banners seen from localStorage
 * @return {number} Number of banners seen
 */
frb.getSeenCount = function () {

    // Force with URL parameter &quot; , &quot;'&quot; , &quot;impression&quot; , &quot;'&quot; , &quot;
    if ( typeof URLSearchParams === &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot; ) { // not available in old browsers
        var urlParams = new URLSearchParams( window.location.search );
        if ( urlParams.has( &quot; , &quot;'&quot; , &quot;impression&quot; , &quot;'&quot; , &quot; ) ) {
            return urlParams.get( &quot; , &quot;'&quot; , &quot;impression&quot; , &quot;'&quot; , &quot; );
        }
    }

    try {
        if ( localStorage ) {
            var identifier = mw.centralNotice.internal.state.campaign.mixins.impressionDiet.cookieName,
                lsName = &quot; , &quot;'&quot; , &quot;CentralNoticeKV|global|impression_diet_&quot; , &quot;'&quot; , &quot; + identifier,
                diet = JSON.parse( localStorage.getItem( lsName ) );
            if ( diet ) {
                return diet.val.seenCount;
            }
        }
    } catch ( ex ) {
        // do nothing - localStorage is configured not to let us read it, or mixin not set
        return;
    }
};

/**
 * Helper function to do text replacements and wrap them in correct class
 * 
 * @param  {RegExp} regex       Regular expression to replace
 * @param  {string} replacement String to replace it with
 */
frb.textReplace = function( regex, replacement ) {
    $( &quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot; ).each( function( index ) {
        var newHtml = $( this ).html();
        newHtml = newHtml.replace( regex, &quot; , &quot;'&quot; , &quot;&lt;span class=&quot;frb-replaced&quot;>&quot; , &quot;'&quot; , &quot; + replacement + &quot; , &quot;'&quot; , &quot;&lt;/span>&quot; , &quot;'&quot; , &quot; );
        $( this ).html( newHtml );
    });
};

/**
 * Replace elements with preset ask string amounts
 *
 * e.g. class=&quot;frb-replace-amt-ps4&quot; will be replaced with amount #4, currently $25 in the US
 *
 * @param  {string} currency - currency code e.g. &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot;
 * @param  {string} country  - country code  e.g. &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;
 * @param  {string} language - language code e.g. &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; For symbol formatting
 */
frb.replaceCustomAmounts = function( currency, country, language ) {
    var amountOptions = frb.pickAmounts( frb.amounts.options7, currency, country );

    // Old style element replacements
    $( &quot; , &quot;'&quot; , &quot;.frb [class^=&quot;frb-replace-amt-ps&quot;]&quot; , &quot;'&quot; , &quot; ).each( function() {
        var i = $( this ).attr( &quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot; ).replace( &quot; , &quot;'&quot; , &quot;frb-replace-amt-ps&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ),
            amount = amountOptions[ i - 1 ],
            formattedAmount = frb.formatCurrency( currency, amount, language );
        $( this ).html( &quot; , &quot;'&quot; , &quot;&lt;frb-amt>&quot; , &quot;'&quot; , &quot; + formattedAmount + &quot; , &quot;'&quot; , &quot;&lt;/frb-amt>&quot; , &quot;'&quot; , &quot; );
    });

    // Text replacements e.g. %amount-4%
    // There is probably a more efficient way to do this, but it&quot; , &quot;'&quot; , &quot;s at least fairly simple
    for (var i = 0; i &lt; amountOptions.length; i++) {
        var amount = amountOptions[i],
            formattedAmount,
            regex = new RegExp( &quot; , &quot;'&quot; , &quot;%amount-&quot; , &quot;'&quot; , &quot; + (i+1) + &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;gi&quot; , &quot;'&quot; , &quot; );
        if ( frb.textAmountIsoCountries.includes( country ) ) {
            formattedAmount = frb.formatCurrency( undefined, amount, language ) + &quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot; + currency;
        } else {
            formattedAmount = frb.formatCurrency( currency, amount, language );
        }
        frb.textReplace( regex, formattedAmount );
    }
};

/**
 * Get today&quot; , &quot;'&quot; , &quot;s date like &quot;December 3&quot; - English only for now
 * 
 * @return {string} Today&quot; , &quot;'&quot; , &quot;s date as a string
 */
frb.getDateString = function() {
    var date = new Date(),
        locale = mw.centralNotice.data.uselang + &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; + mw.centralNotice.data.country;
    return date.toLocaleString( locale, { day: &quot; , &quot;'&quot; , &quot;numeric&quot; , &quot;'&quot; , &quot;, month: &quot; , &quot;'&quot; , &quot;long&quot; , &quot;'&quot; , &quot; } );
};

frb.noRecurringCountries = [&quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;];
frb.ccAdyenCountries     = [&quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;];

/* These countries use potentially ambiguous $ sign.
Use ISO code instead in text (but still $ for buttons) */
frb.textAmountIsoCountries = [&quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;];

$(function() {

    if ( mw.centralNotice.adminUi ) { // T262693
        return;
    }

    var language = mw.centralNotice.data.uselang;
    var variantLanguage; // for pt-br and es-419, note we can only use these for certain links
    var country  = mw.centralNotice.data.country;
    var currency = frb.getCurrency(country);

    if ( language === &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot; &amp;&amp; country === &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; ) {
        variantLanguage = &quot; , &quot;'&quot; , &quot;pt-br&quot; , &quot;'&quot; , &quot;;
    } else if ( language === &quot; , &quot;'&quot; , &quot;es&quot; , &quot;'&quot; , &quot; &amp;&amp; [&quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot;].indexOf( country ) !== -1 ) {
        variantLanguage = &quot; , &quot;'&quot; , &quot;es-419&quot; , &quot;'&quot; , &quot;;
    } else {
        variantLanguage = language;
    }

    // Payment methods
    frb.localizeMethods(country);
    frb.checkMethodOutages(country);

    // Preset amounts
    frb.replaceCustomAmounts( currency, country, language );

    // Basic replacements
    $(&quot; , &quot;'&quot; , &quot;.frb-replace-currencysymbol&quot; , &quot;'&quot; , &quot;).text( frb.formatCurrency( currency, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, language ).replace(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) );
    $(&quot; , &quot;'&quot; , &quot;.frb-replace-currencycode&quot; , &quot;'&quot; , &quot;).text( currency );

    // Country name
    var countryName;
    if ( frb.countryNames[language] ) {
        countryName = frb.countryNames[language][country] || frb.countryNames.en[country];
    } else {
        countryName = frb.countryNames.en[country];
    }
    $( &quot; , &quot;'&quot; , &quot;.frb-replace-countryname&quot; , &quot;'&quot; , &quot; ).text( countryName );
    frb.textReplace( /%country%/gi, countryName );

    // &quot;in COUNTRY&quot; or equivalent
    var inCountryName;
    if ( frb.inCountryNames[language] ) {
        inCountryName = frb.inCountryNames[language][country] || frb.inCountryNames.en[country];
    } else {
        inCountryName = frb.inCountryNames.en[country];
    }
    $( &quot; , &quot;'&quot; , &quot;.frb-replace-incountryname&quot; , &quot;'&quot; , &quot; ).text( inCountryName );
    frb.textReplace( /%in-country%/gi, inCountryName );

    // Day of week
    // TODO: Replace these with date.toLocaleString so we can drop frb.dayNames? 
    //       Might still need some ways to deal with &quot;this&quot; and capitalization
    var now = new Date();
    var dayNumber = now.getDay();
    var capitalizeText = function( text ) {
        // Capitalize first letter, for use at start of sentence
        return text.charAt(0).toUpperCase() + text.slice(1);
    };

    if ( $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek, .frb-replace-dayofweek-capitalize&quot; , &quot;'&quot; , &quot;).length > 0 ) {
        if ( frb.dayNames[language] ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek&quot; , &quot;'&quot; , &quot;).text( frb.dayNames[language][dayNumber] );
            $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek-capitalize&quot; , &quot;'&quot; , &quot;).text( capitalizeText( frb.dayNames[language][dayNumber] ) );
        } else {
            console.log(&quot; , &quot;'&quot; , &quot;Warning: banner should contain a day of the week, but no translations found.&quot; , &quot;'&quot; , &quot;);
        }
    }

    if ( $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek-this, .frb-replace-dayofweek-this-capitalize&quot; , &quot;'&quot; , &quot;).length > 0 ) {
        if ( frb.dayNamesThis[language] ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek-this&quot; , &quot;'&quot; , &quot;).text( frb.dayNamesThis[language][dayNumber] );
            $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek-this-capitalize&quot; , &quot;'&quot; , &quot;).text( capitalizeText( frb.dayNamesThis[language][dayNumber] ) );
        } else {
            console.log(&quot; , &quot;'&quot; , &quot;Warning: banner should contain &quot;this DAY&quot;, but no translations found.&quot; , &quot;'&quot; , &quot;);
        }
    }

    // Simple %weekday% text replacement
    try {
        if ( frb.dayNames[language] ) {
            frb.textReplace( /%weekday%/gi, frb.dayNames[language][dayNumber] );
        } else {
            frb.textReplace( /%weekday%/gi, frb.dayNames[&quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;][dayNumber] );
        }
    } catch ( error ) {
        console.error( error );
    }

    // Replace %date% with today&quot; , &quot;'&quot; , &quot;s date e.g. &quot;December 3&quot;
    try {
        frb.textReplace( /%date%/gi, frb.getDateString() );
    } catch ( error ) {
    	console.log( error );
    }

    // Capitalize
    $(&quot; , &quot;'&quot; , &quot;.frb-capitalize&quot; , &quot;'&quot; , &quot;).text(function( index, text ) {
        return text.charAt(0).toUpperCase() + text.slice(1);
    });

    // Replace %average%, %minimum% and %amount%
    var average = frb.pickAmounts( frb.amounts.averages, currency, country ),
        ifEveryone = frb.pickAmounts( frb.amounts.ifEveryone, currency, country ),
        avgString,
        ifString;

    if ( frb.textAmountIsoCountries.indexOf(country) !== -1 ) {
        avgString = frb.formatCurrency( undefined, average, language ) + &quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot; + currency;
        ifString  = frb.formatCurrency( undefined, ifEveryone, language ) + &quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot; + currency;
    } else {
        avgString = frb.formatCurrency( currency, average, language ).replace( /\.$/, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ); // strip any period from end for use in running text
        ifString  = frb.formatCurrency( currency, ifEveryone, language ).replace( /\.$/, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; );
    }
    frb.textReplace( /%average%/gi, avgString );
    frb.textReplace( /%minimum%/gi, ifString );
    frb.textReplace( /%amount%/gi,  ifString );

    /**
     * Call a function on every text node contained by a root node.
     *
     * Used so we can do text replacements without accidentally clobbering html and scripts
     *
     * @param  {Node}     rootNode The Node object whose descendants will be recursed through
     * @param  {Function} callback Callback function that receives a Node as its only argument
     */
    function eachTextNode( rootNode, callback ) {
        for ( var node = rootNode.firstChild; node !== null; node = node.nextSibling ) {
            if ( node.nodeType === Node.TEXT_NODE ) {
                callback( node );
            } else if ( node.nodeType === Node.ELEMENT_NODE ) {
                eachTextNode( node, callback );
            }
        }
    }

    // French spacing: replace space before punctuation with &amp;nbsp;
    if ( language === &quot; , &quot;'&quot; , &quot;fr&quot; , &quot;'&quot; , &quot; ) {
        var bannerRootElements = document.getElementsByClassName( &quot; , &quot;'&quot; , &quot;frb&quot; , &quot;'&quot; , &quot; );
        for ( var i = 0; i &lt; bannerRootElements.length; i++ ) {
            eachTextNode( bannerRootElements[i], function( node ) {
                node.textContent = node.textContent.replace( / ([!?;:%])/g, &quot; , &quot;'&quot; , &quot;\u00a0$1&quot; , &quot;'&quot; , &quot; );
            });
        }
    }

    // Links (in smallprint) TODO: merge with frb.makeLink()
    $(&quot; , &quot;'&quot; , &quot;.frb-localize-links a&quot; , &quot;'&quot; , &quot;).each(function() {
        // Add parameters for LandingCheck
        var uri = new mw.Uri( $(this).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;) );
        uri.extend({
            country:      country,
            language:     variantLanguage,
            uselang:      variantLanguage,
            utm_medium:   &quot; , &quot;'&quot; , &quot;sitenotice&quot; , &quot;'&quot; , &quot;,
            utm_campaign: mw.centralNotice.data.campaign || &quot; , &quot;'&quot; , &quot;test&quot; , &quot;'&quot; , &quot;,
            utm_source:   mw.centralNotice.data.banner
        });
        $(this).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, uri.toString());
        $(this).attr(&quot; , &quot;'&quot; , &quot;target&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;); // Make links open in new tab
    });

    // Add links
    frb.makeLink( &quot; , &quot;'&quot; , &quot;.frb-link-privacy&quot; , &quot;'&quot; , &quot;, variantLanguage, &quot; , &quot;'&quot; , &quot;https://foundation.wikimedia.org/wiki/Special:LandingCheck?basic=true&amp;landing_page=Donor_privacy_policy&quot; , &quot;'&quot; , &quot; );
    frb.makeLink( &quot; , &quot;'&quot; , &quot;.frb-link-tax&quot; , &quot;'&quot; , &quot;,     variantLanguage, &quot; , &quot;'&quot; , &quot;https://donate.wikimedia.org/wiki/Special:LandingCheck?basic=true&amp;landing_page=Tax_deductibility&quot; , &quot;'&quot; , &quot; );
    frb.makeLink( &quot; , &quot;'&quot; , &quot;.frb-link-cancel&quot; , &quot;'&quot; , &quot;,  variantLanguage, &quot; , &quot;'&quot; , &quot;https://donate.wikimedia.org/wiki/Special:LandingCheck?basic=true&amp;landing_page=Cancel_or_change_recurring_giving&quot; , &quot;'&quot; , &quot; );

    // Legal text variants
    if (country === &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;) {
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-US&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-nonUS, .frb-legal-NL&quot; , &quot;'&quot; , &quot;).hide();
    } else if (country === &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;) {
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-NL&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-US, .frb-legal-nonUS&quot; , &quot;'&quot; , &quot;).hide();
    } else {
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-nonUS&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-US, .frb-legal-NL&quot; , &quot;'&quot; , &quot;).hide();
    }

    // Quick hack for American/British/Commonwealth English differences
    if ( country === &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; ) {
        $(&quot; , &quot;'&quot; , &quot;.frb-lang-enUS&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;.frb-lang-en5C&quot; , &quot;'&quot; , &quot;).hide();
    } else {
        $(&quot; , &quot;'&quot; , &quot;.frb-lang-enUS&quot; , &quot;'&quot; , &quot;).hide();
        $(&quot; , &quot;'&quot; , &quot;.frb-lang-en5C&quot; , &quot;'&quot; , &quot;).show();
    }

    // Add this so they get white-space: nowrap from CSS
    $(&quot; , &quot;'&quot; , &quot;.frb-ptf-fee, .frb-ptf-total, .frb-upsell-ask&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-replaced&quot; , &quot;'&quot; , &quot;);

    // Where Remind Me Later should be shown
    var rmlCountries = [&quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot;,
                        &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;];
    var rmlLanguages = [&quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nl&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ja&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;it&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sv&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;cs&quot; , &quot;'&quot; , &quot;];
    var rmlEnabled = !frb.isEndowment &amp;&amp; rmlCountries.indexOf(country) !== -1 &amp;&amp; rmlLanguages.indexOf(language) !== -1;

    if ( rmlEnabled ) {
        $(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-rml-enabled&quot; , &quot;'&quot; , &quot;);
    } else {
        $(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-rml-disabled&quot; , &quot;'&quot; , &quot;);
    }

});

/* == end of MediaWiki:FundraisingBanners/LocalizeJS-2022.js == */
/* eslint-env es6 */
var frb = frb || {};

/* Based on github:braintree/braintree-web/src/venmo/shared/supports-venmo.js */
frb.isVenmoSupported = function(options) {
  var options = options || {
    allowNewBrowserTab: false,
    allowWebviews: true,
    allowDesktop: true,
    allowDesktopWebLogin: true
  };
  var ua = window.navigator.userAgent;

  var merchantAllowsReturningToNewBrowserTab,
    merchantAllowsWebviews,
    merchantAllowsDesktopBrowsers;
  var isMobileDevice = isAndroid() || isIos();
  var isAndroidChrome = isAndroid() &amp;&amp; isChrome();
  var isMobileDeviceThatSupportsReturnToSameTab = isIosSafari() || isAndroidChrome;
  var isKnownUnsupportedMobileBrowser = isIosChrome() || isFacebookOwnedBrowserOnAndroid() || isSamsung();

  options = options || {};
  // NEXT_MAJOR_VERSION allowDesktop will default to true, but can be opted out
  merchantAllowsDesktopBrowsers =
    (options.allowDesktopWebLogin || options.allowDesktop) === true;
  merchantAllowsReturningToNewBrowserTab = options.hasOwnProperty(
    &quot;allowNewBrowserTab&quot;
  )
    ? options.allowNewBrowserTab
    : true;
  // NEXT_MAJOR_VERSION webviews are not supported, except for the case where
  // the merchant themselves is presenting venmo in a webview using the deep
  // link url to get back to their app. For the next major version, we should
  // just not have this option and instead require the merchant to determine
  // if the venmo button should be displayed when presenting it in the
  // merchant&quot; , &quot;'&quot; , &quot;s app via a webview.
  merchantAllowsWebviews = options.hasOwnProperty(&quot;allowWebviews&quot;)
    ? options.allowWebviews
    : true;

  if (isKnownUnsupportedMobileBrowser) {
    return false;
  }

  if (
    !merchantAllowsWebviews &amp;&amp;
    (isAndroidWebview() || isIosWebview())
  ) {
    return false;
  }

  if (!isMobileDevice) {
    return merchantAllowsDesktopBrowsers;
  }

  if (!merchantAllowsReturningToNewBrowserTab) {
    return isMobileDeviceThatSupportsReturnToSameTab;
  }

  return isMobileDevice;

  /* -- functions mostly from github:braintree/browser-detection library -- */

  function isAndroid() {
    return /Android/i.test(ua);
  }

  function isIos(checkIpadOS = true) {
    const iOsTest = /iPhone|iPod|iPad/i.test(ua);
    return checkIpadOS ? iOsTest || isIpadOS() : iOsTest;
  }

  function isIpadOS() {
    // &quot;ontouchend&quot; is used to determine if a browser is on an iPad, otherwise
    // user-agents for iPadOS behave/identify as a desktop browser
    return /Mac|iPad/i.test(ua) &amp;&amp; &quot;ontouchend&quot; in window.document;
  }

  function isEdge() {
    return ua.indexOf(&quot;Edge/&quot;) !== -1 || ua.indexOf(&quot;Edg/&quot;) !== -1;
  }

  function isSamsung() {
    return /SamsungBrowser/i.test(ua);
  }

  function isDuckDuckGo() {
    return ua.indexOf(&quot;DuckDuckGo/&quot;) !== -1;
  }

  function isOpera() {
    return (
      ua.indexOf(&quot;OPR/&quot;) !== -1 ||
      ua.indexOf(&quot;Opera/&quot;) !== -1 ||
      ua.indexOf(&quot;OPT/&quot;) !== -1
    );
  }

  function isSilk() {
    return ua.indexOf(&quot;Silk/&quot;) !== -1;
  }

  function isChrome() {
    return (
      (ua.indexOf(&quot;Chrome&quot;) !== -1 || ua.indexOf(&quot;CriOS&quot;) !== -1) &amp;&amp;
      !isEdge() &amp;&amp;
      !isSamsung() &amp;&amp;
      !isDuckDuckGo() &amp;&amp;
      !isOpera() &amp;&amp;
      !isSilk()
    );
  }

  function isIosFirefox() {
    return /FxiOS/i.test(ua);
  }

  function isWebkit() {
    const webkitRegexp = /webkit/i;
    return webkitRegexp.test(ua);
  }

  function isIosChrome() {
    return ua.indexOf(&quot;CriOS&quot;) > -1;
  }

  function isFacebook() {
    return ua.indexOf(&quot;FBAN&quot;) > -1;
  }

  function isIosSafari() {
    return (
      isIos() &amp;&amp;
      isWebkit() &amp;&amp;
      !isIosChrome() &amp;&amp;
      !isIosFirefox() &amp;&amp;
      !isFacebook()
    );
  }

  function isFacebookOwnedBrowserOnAndroid() {
    var e = ua.toLowerCase();
    return -1 &lt; e.indexOf(&quot;huawei&quot;) &amp;&amp; -1 &lt; e.indexOf(&quot;fban&quot;) || isAndroid() &amp;&amp; (-1 &lt; e.indexOf(&quot;fb_iab&quot;) || -1 &lt; e.indexOf(&quot;instagram&quot;));
  }

  function isSamsungBrowser() {
    return /SamsungBrowser/i.test(ua);
  }

  function isAndroidWebview() {
    return isAndroid() &amp;&amp; -1 &lt; ua.toLowerCase().indexOf(&quot;wv&quot;);
  }

  function isGoogleSearchApp() {
    return /\bGSA\b/.test(ua);
  }

  function isIosGoogleSearchApp() {
    return isIos() &amp;&amp; isGoogleSearchApp();
  }

  function isIosWebview() {
    if (isIos()) {
      // The Google Search iOS app is technically a webview and doesn&quot; , &quot;'&quot; , &quot;t support popups.
      if (isIosGoogleSearchApp()) {
        return true;
      }
      // Historically, a webview could be identified by the presence of AppleWebKit and _no_ presence of Safari after.
      return /.+AppleWebKit(?!.*Safari)/i.test(ua);
    }
    return false;
  }
};

$(function() {

    if ( mw.centralNotice.adminUi || !frb.supportedBrowser ) { // T262693
        return;
    }

    var language = mw.centralNotice.data.uselang;
    var country  = mw.centralNotice.data.country;
    var currency = frb.getCurrency(country);
    var validAmount;
    var validMethod;
    var validOptin;
    var form = document.getElementById(&quot; , &quot;'&quot; , &quot;frb-form&quot; , &quot;'&quot; , &quot;);

    var animationDuration = frb.reduceMotion ? 0 : 400;

    mw.loader.using([&quot; , &quot;'&quot; , &quot;mediawiki.util&quot; , &quot;'&quot; , &quot;]).then(function() {
        frb.rml.init();
    });

    frb.initAmountOptions();
    frb.localizeAmountOptions( frb.amounts.options7, currency, country, language, true );
    frb.localizeErrors();

    frb.storedOptions = {};
    frb.extraData = {};

    frb.setMethod = function (options, frequency) {
        frb.storedOptions = options;

        if( frequency === &quot; , &quot;'&quot; , &quot;no-monthly&quot; , &quot;'&quot; , &quot; ) {
            $(&quot; , &quot;'&quot; , &quot;#frb-frequency-monthly&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
        } else {
            $(&quot; , &quot;'&quot; , &quot;#frb-frequency-monthly&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
        }
    };

    frb.updateUpsellAsk = function(isOtherAmountStep) {
        var amount, feeAmount, upsellAmount,
            list = frb.amounts.monthlySuggest[currency];

        if ( list === undefined ) {
            console.log(&quot; , &quot;'&quot; , &quot;No monthlySuggest amounts found for &quot; , &quot;'&quot; , &quot; + currency);
            return;
        }

        // If user is on third step (write a different amount) then get monthly amount if not, the the first form amount
        if (isOtherAmountStep !== undefined) {
            amount = frb.getMonthlyAmount();
        } else {
            amount = frb.getAmount(form);
        }

        // If PTF is checked when we need to calculate the fee for that amount
        if ( $(&quot; , &quot;'&quot; , &quot;#frb-ptf-checkbox&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;) ) {
            amount = amount + frb.calculateFee(amount);
        }

        for (var i = list.length - 1; i >= 0; i--) {
            if ( amount &lt;= list[i][0] ) {
                upsellAmount = list[i][1];
            }
        }

        // If user is in the upsell (second step) then the form.otherMonthlyAmount.value will be updated with the upsellAmount calculated
        if (isOtherAmountStep === undefined) {
            form.otherMonthlyAmount.value = upsellAmount;
        }

        // A formatted value will be returned
        var upsellAmountFormatted = frb.formatCurrency(currency, upsellAmount, language);

        // The value of the amount will be updated only if the user is in the upsell (second step)
        if (isOtherAmountStep === undefined) {
            $(&quot; , &quot;'&quot; , &quot;.frb-upsell-ask&quot; , &quot;'&quot; , &quot;).text(upsellAmountFormatted);
        }
    };

    $(&quot; , &quot;'&quot; , &quot;.frb-amounts input:not(#input_amount_other)&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input change&quot; , &quot;'&quot; , &quot;, function(e) {

        // Deal with https://phabricator.wikimedia.org/T191417
        if ( this.value === &quot;&quot; ) {
            return;
        }

        if ( frb.validateAmount() ) {
            validAmount = 1;
        } else {
            validAmount = 0;
        }
        frb.updateFeeDisplay();
        frb.activateCTA();
    });

    $(&quot; , &quot;'&quot; , &quot;.frb-methods&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        $(&quot; , &quot;'&quot; , &quot;.frb-methods&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-method&quot; , &quot;'&quot; , &quot;).hide();
        $(&quot; , &quot;'&quot; , &quot;.frb-optin&quot; , &quot;'&quot; , &quot;).slideDown( animationDuration );
        validMethod = 1;
        frb.activateCTA();
    });

    // Opt-in interaction
    $(&quot; , &quot;'&quot; , &quot;.frb-optin&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        $(&quot; , &quot;'&quot; , &quot;.frb-optin&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-optin&quot; , &quot;'&quot; , &quot;).hide();
        if ( $(&quot; , &quot;'&quot; , &quot;#frb-optin-no&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;) ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-optin-no-prompt&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-positive&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-optin-no-prompt&quot; , &quot;'&quot; , &quot;).slideDown( animationDuration );
        } else {
            $(&quot; , &quot;'&quot; , &quot;.frb-optin-no-prompt&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;is-positive&quot; , &quot;'&quot; , &quot;);
        }
        validOptin = 1;
        frb.activateCTA();
    });

    // Go to the next step of the form
    $(&quot; , &quot;'&quot; , &quot;#frb-continue&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        e.preventDefault();
        var status = {amount: false, method: false};

        // Validate amount
        if( frb.validateAmount() ){
            status.amount = true;
        } else {
            frb.extraData.validateError = 1;
        }

        // Validate method
        if ($(&quot; , &quot;'&quot; , &quot;input[name=&quot;frb-methods&quot;]:checked&quot; , &quot;'&quot; , &quot;).length === 1) {
            status.method = true;
        } else {
            frb.extraData.validateError = 1;
            $(&quot; , &quot;'&quot; , &quot;.frb-methods&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-error-method&quot; , &quot;'&quot; , &quot;).show();
        }

        if (status.amount === true &amp;&amp; status.method === true) {

            frb.updateUpsellAsk();

            $(&quot; , &quot;'&quot; , &quot;.frb-rml-link, .frb-rml&quot; , &quot;'&quot; , &quot;).hide();

            if ( frb.optinRequired ) {
                frb.showStep(&quot; , &quot;'&quot; , &quot;optin&quot; , &quot;'&quot; , &quot;);
                setTimeout( () => {
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).css({ &quot; , &quot;'&quot; , &quot;visibility&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot; });
                }, 300 );
            } else if ( frb.shouldShowMonthlyConvert() ) {
                frb.showStep(&quot; , &quot;'&quot; , &quot;upsell&quot; , &quot;'&quot; , &quot;);
                setTimeout( () => {
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).css({ &quot; , &quot;'&quot; , &quot;visibility&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot; });
                }, 300 );
            } else {
                frb.submitForm( frb.storedOptions );
            }
        }
    });

    /* -- Back buttons -- */
    $(&quot; , &quot;'&quot; , &quot;.frb-step-optin .frb-back&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        frb.showStep(&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;);
        return false;
    });

    $(&quot; , &quot;'&quot; , &quot;.frb-step-upsell .frb-back&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        if ( frb.optinRequired ) {
            frb.showStep(&quot; , &quot;'&quot; , &quot;optin&quot; , &quot;'&quot; , &quot;);
        } else {
            frb.showStep(&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;);
        }
        return false;
    });

    $(&quot; , &quot;'&quot; , &quot;.frb-step-monthly-diff-amt .frb-back&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        form.otherMonthlyAmount.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        frb.updateUpsellAsk();
        validAmount = 1;
        frb.activateCTA();
        frb.toggleMonthly(false);
        frb.showStep(&quot; , &quot;'&quot; , &quot;upsell&quot; , &quot;'&quot; , &quot;);
        return false;
    });

    // Donate monthly other amount
    $(&quot; , &quot;'&quot; , &quot;.frb-monthly-diff-amt-link&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        form.otherMonthlyAmount.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        validAmount = 0;
        frb.activateCTA();
        frb.toggleMonthly(true);
        frb.showStep(&quot; , &quot;'&quot; , &quot;monthly-diff-amt&quot; , &quot;'&quot; , &quot;);
        return false;
    });

    // Validate monthly other amount
    $(&quot; , &quot;'&quot; , &quot;#frb-amt-monthly-other-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input change&quot; , &quot;'&quot; , &quot;, function(e) {
        if ( frb.validateMonthlyAmount() ) {
            validAmount = 1;
            frb.updateUpsellAsk(true);
        } else {
            validAmount = 0;
        }
        frb.activateCTA();
    });

    frb.getMonthlyAmount = function() {
        var amount = null;

        // Check the &quot;monthly other&quot; amount box
        if (form.otherMonthlyAmount.value !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            var otherMonthlyAmount = form.otherMonthlyAmount.value;
            otherMonthlyAmount = otherMonthlyAmount.replace(/[,.](\d)$/, &quot; , &quot;'&quot; , &quot;:$10&quot; , &quot;'&quot; , &quot;);
            otherMonthlyAmount = otherMonthlyAmount.replace(/[,.](\d)(\d)$/, &quot; , &quot;'&quot; , &quot;:$1$2&quot; , &quot;'&quot; , &quot;);
            otherMonthlyAmount = otherMonthlyAmount.replace(/[$£€¥,.]/g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            otherMonthlyAmount = otherMonthlyAmount.replace(/:/, &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;);
            amount = otherMonthlyAmount;
        }

        amount = parseFloat(amount);

        if ( isNaN(amount) ) {
            return 0;
        } else {
            var totalMonthlyAmountFormatted = frb.formatCurrency(currency, amount, language);
            $(&quot; , &quot;'&quot; , &quot;.frb-monthly-total&quot; , &quot;'&quot; , &quot;).text(totalMonthlyAmountFormatted);

            return amount;
        }
    };

    frb.validateMonthlyAmount = function() {
        var amount = frb.getMonthlyAmount();
        var currency  = frb.getCurrency( mw.centralNotice.data.country );
        var minAmount = frb.amounts.minimums[ currency ];

        if ( amount === null || isNaN(amount) || amount &lt;= 0 || amount &lt; minAmount ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-error-bigamount&quot; , &quot;'&quot; , &quot;).hide();
            $(&quot; , &quot;'&quot; , &quot;.frb-error-smallamount&quot; , &quot;'&quot; , &quot;).show();
            return false;
        } else if ( amount > frb.maxUSD * minAmount ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-error-bigamount&quot; , &quot;'&quot; , &quot;).show();
            return false;
        } else {
            $(&quot; , &quot;'&quot; , &quot;.frb-error-smallamount, .frb-error-bigamount&quot; , &quot;'&quot; , &quot;).hide();
            return true;
        }
    };

    frb.submitMonthly = function() {
        frb.extraData.monthlyUpsell = 1;
        frb.extraData.originalAmt = frb.getAmount().toString();

        frb.toggleMonthly(true);
        document.getElementById(&quot; , &quot;'&quot; , &quot;input_amount_other&quot; , &quot;'&quot; , &quot;).checked = true;
        document.getElementById(&quot; , &quot;'&quot; , &quot;frb-ptf-checkbox&quot; , &quot;'&quot; , &quot;).checked = false;
        form.otherAmount.value = form.otherMonthlyAmount.value;
        frb.submitForm(frb.storedOptions);
    };

    // Submit form
    $(&quot; , &quot;'&quot; , &quot;#frb-monthly-donate-yes&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        frb.submitMonthly();
        return false;
    });

    $(&quot; , &quot;'&quot; , &quot;#frb-monthly-donate-no&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        frb.submitForm(frb.storedOptions);
        return false;
    });

    $(&quot; , &quot;'&quot; , &quot;#frb-donate-monthly-other&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function (e) {
        if (frb.validateMonthlyAmount()) {
            frb.submitMonthly();
        }
        return false;
    });

    /**
     * Should we show pre-payment monthly convert?
     *
     * Only if: initial selection is one-time, suggested amount is not 0 (meaning skip),
     * payment method supports monthly, and payment method does not have post-payments monthly convert
     *
     * @returns boolean
     */
     frb.shouldShowMonthlyConvert = function() {
        let postPaymentMethods = [ &quot; , &quot;'&quot; , &quot;cc&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;apple&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;google&quot; , &quot;'&quot; , &quot; ];
        if (
            frb.getRecurring( document.getElementById( &quot; , &quot;'&quot; , &quot;frb-form&quot; , &quot;'&quot; , &quot; ) ) ||
            !frb.shouldShowRecurring( frb.storedOptions, mw.centralNotice.data.country ) ||
            form.otherMonthlyAmount.value == 0 ||
            postPaymentMethods.includes( frb.storedOptions.method )
        ) {
            return false;
        } else {
            return true;
        }
    }

    $(&quot; , &quot;'&quot; , &quot;#frb-donate&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        if ( frb.validateForm( frb.storedOptions) ) {
            if ( frb.shouldShowMonthlyConvert() ) {
                frb.showStep(&quot; , &quot;'&quot; , &quot;upsell&quot; , &quot;'&quot; , &quot;);
            } else {
                frb.submitForm( frb.storedOptions );
            }
        } else {
            frb.extraData.validateError = 1;
        }
        return false;
    });

    // Focus for #input_amount_other
    $(&quot; , &quot;'&quot; , &quot;.frb-amt-other&quot; , &quot;'&quot; , &quot;).click(function() {
        document.getElementById(&quot; , &quot;'&quot; , &quot;input_amount_other&quot; , &quot;'&quot; , &quot;).checked = true;
        frb.updateFeeDisplay();
        $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).focus();
    });

    // Activate #input_amount_other radio when tabbing into #frb-amt-other-input
    $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).focus(function() {
        document.getElementById(&quot; , &quot;'&quot; , &quot;input_amount_other&quot; , &quot;'&quot; , &quot;).checked = true;
        frb.updateFeeDisplay();
    });

    frb.activateCTA = function(){
        if ( validAmount &amp;&amp; validMethod ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-submit-txt-continue&quot; , &quot;'&quot; , &quot;).hide();
            $(&quot; , &quot;'&quot; , &quot;.frb-submit-txt-donate&quot; , &quot;'&quot; , &quot;).show();
            $(&quot; , &quot;'&quot; , &quot;#frb-continue, #frb-monthly-donate-yes, #frb-monthly-donate-no, #frb-donate-monthly-other&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
            if (validOptin) {
                $(&quot; , &quot;'&quot; , &quot;#frb-donate&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
            }
        } else {
            $(&quot; , &quot;'&quot; , &quot;.frb-submit&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        }
    };


    /* --- Nag/minimized banner functionality --- */

    // On Load
    var bannerOuterHeight = $(&quot; , &quot;'&quot; , &quot;.frb-in-article&quot; , &quot;'&quot; , &quot;).outerHeight( true );
    var stickyHeaderTop = bannerOuterHeight + $(&quot; , &quot;'&quot; , &quot;.frb-in-article&quot; , &quot;'&quot; , &quot;).offset().top + 200;

    frb.initNag = function() {

        // Intercept TOC clicks, and account for nag height
        $(&quot; , &quot;'&quot; , &quot;#toc ul > li a&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click.frb&quot; , &quot;'&quot; , &quot;, function(e) {
            e.preventDefault();

            var anchor = $(this).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;).replace(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            anchor = $(&quot;[id=&quot; , &quot;'&quot; , &quot;&quot;+$.escapeSelector(anchor)+&quot;&quot; , &quot;'&quot; , &quot;]&quot;);

            var offsetTop = anchor.offset().top - $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).outerHeight();
            $(&quot; , &quot;'&quot; , &quot;body, html&quot; , &quot;'&quot; , &quot;).animate({ scrollTop: offsetTop }, 10);
            window.location.hash = $(this).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;);
        });

        // Scroll to section, accounting for nag height
        if ( window.location.hash ) {
            var offsetTop;
            var hash = decodeURI(window.location.hash).replace(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            hash = $(&quot;[id=&quot; , &quot;'&quot; , &quot;&quot;+$.escapeSelector(hash)+&quot;&quot; , &quot;'&quot; , &quot;]&quot;);
            if ( hash.offset() ) { // T281547
                offsetTop = hash.offset().top + bannerOuterHeight - $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).outerHeight();
                $(&quot; , &quot;'&quot; , &quot;body, html&quot; , &quot;'&quot; , &quot;).animate( { scrollTop: offsetTop }, 100 );
            }
        }

        $(window).on(&quot; , &quot;'&quot; , &quot;resize.frb&quot; , &quot;'&quot; , &quot;, function() {
            bannerOuterHeight = $(&quot; , &quot;'&quot; , &quot;.frb-in-article&quot; , &quot;'&quot; , &quot;).outerHeight( true );
            stickyHeaderTop = bannerOuterHeight + $(&quot; , &quot;'&quot; , &quot;.frb-in-article&quot; , &quot;'&quot; , &quot;).offset().top + 200;
        });

        function scrollFunction() {
            if( $(window).scrollTop() > stickyHeaderTop ) {
                if ( !frb.fixed ) {
                    $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).show();
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml-form-wrapper&quot; , &quot;'&quot; , &quot;).hide();
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;.frb-nag .frb-form-wrapper&quot; , &quot;'&quot; , &quot;);
                }
            } else {
                if ( frb.fixed ) {
                    $(&quot; , &quot;'&quot; , &quot;.frb-prevent-page-jump&quot; , &quot;'&quot; , &quot;)
                        .removeClass(&quot; , &quot;'&quot; , &quot;frb-in-article&quot; , &quot;'&quot; , &quot;)
                        .hide();
                    $(&quot; , &quot;'&quot; , &quot;#frb-main&quot; , &quot;'&quot; , &quot;)
                        .removeClass(&quot; , &quot;'&quot; , &quot;frb-fixed&quot; , &quot;'&quot; , &quot;)
                        .addClass(&quot; , &quot;'&quot; , &quot;frb-in-article&quot; , &quot;'&quot; , &quot;)
                        .trigger(&quot; , &quot;'&quot; , &quot;unFixed&quot; , &quot;'&quot; , &quot;);
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;#frb-main .frb-form-wrapper&quot; , &quot;'&quot; , &quot;);
                    frb.fixed = false;
                } else {
                    $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).hide();
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;#frb-main .frb-form-wrapper&quot; , &quot;'&quot; , &quot;);
                }
            }
        }

        $(window).on(&quot; , &quot;'&quot; , &quot;load.frb scroll.frb resize.frb&quot; , &quot;'&quot; , &quot;, function() {
            scrollFunction();
        });

        frb.clickNag = function(e) {
            frb.extraData.clickedNag = 1;

            if ( window.innerHeight &lt; document.getElementById(&quot; , &quot;'&quot; , &quot;frb-main&quot; , &quot;'&quot; , &quot;).offsetHeight ) {
                // Window height too short for fixing position, just jump to main banner
                window.scrollTo(0, 0);
                return false;
            }

            // Add spacer to prevent jump
            var inArticleHeight = $(&quot; , &quot;'&quot; , &quot;#frb-main&quot; , &quot;'&quot; , &quot;).outerHeight();
            $(&quot; , &quot;'&quot; , &quot;.frb-prevent-page-jump&quot; , &quot;'&quot; , &quot;)
                .height( inArticleHeight )
                .addClass(&quot; , &quot;'&quot; , &quot;frb-in-article&quot; , &quot;'&quot; , &quot;) // So that it can be used for stickyHeader calcs
                .show();

            $(&quot; , &quot;'&quot; , &quot;#frb-main&quot; , &quot;'&quot; , &quot;)
                .removeClass(&quot; , &quot;'&quot; , &quot;frb-in-article&quot; , &quot;'&quot; , &quot;)
                .addClass(&quot; , &quot;'&quot; , &quot;frb-fixed&quot; , &quot;'&quot; , &quot;);

            $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;#frb-main .frb-form-wrapper&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).hide();

            frb.fixed = true;
            return false;
        };

        $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).on( &quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, frb.clickNag );
        $(&quot; , &quot;'&quot; , &quot;#nag-yes-btn&quot; , &quot;'&quot; , &quot;).on( &quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, frb.clickNag );

        $(&quot; , &quot;'&quot; , &quot;#nag-rml-btn&quot; , &quot;'&quot; , &quot;).on( &quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
            // Add &quot; , &quot;'&quot; , &quot;_nag&quot; , &quot;'&quot; , &quot; to RML source value
            $(&quot; , &quot;'&quot; , &quot;#frb-rml-form input[name=&quot;rml_source&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;B2324_121810_en6C_dsk_p1_lg_txt_169C_nag&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-rml-displayed&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;); // Remove so they can interact with RML
            $(&quot; , &quot;'&quot; , &quot;#frb-rml-email&quot; , &quot;'&quot; , &quot;).focus();
            $(&quot; , &quot;'&quot; , &quot;.frb-rml-close-wrapper&quot; , &quot;'&quot; , &quot;).hide();
        });

    };

    $(&quot; , &quot;'&quot; , &quot;.back-rml&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;frb-rml-displayed&quot; , &quot;'&quot; , &quot;);
    });

    // Close inline rml form on click or return
    $(&quot; , &quot;'&quot; , &quot;.frb-rml-close&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function (e) {
        $(&quot; , &quot;'&quot; , &quot;.frb-rml-form-wrapper&quot; , &quot;'&quot; , &quot;).hide();
        e.stopPropagation();
    });

    $(&quot; , &quot;'&quot; , &quot;.frb-close&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function (e) {
        frb.hide();
        frb.altSetHideCookie( &quot; , &quot;'&quot; , &quot;close&quot; , &quot;'&quot; , &quot;, frb.HIDE_DURATION_CLOSE );
        frb.showSidebarTooltip();
        return false;
    });

    // Open already donated modal
    $(&quot; , &quot;'&quot; , &quot;.modal-open&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        $(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;button.modal-close-x&quot; , &quot;'&quot; , &quot;).focus();
    });

    // Close already donated modal
    $(&quot; , &quot;'&quot; , &quot;.modal-close&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        closeModal(e);
        $(&quot; , &quot;'&quot; , &quot;.modal-open&quot; , &quot;'&quot; , &quot;).focus();
    });

    $(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).click(function(e) {
        const modalContainer = $(&quot; , &quot;'&quot; , &quot;.modal-container&quot; , &quot;'&quot; , &quot;);

        if (!modalContainer.is(e.target) &amp;&amp; modalContainer.has(e.target).length === 0) {
            closeModal(e);
            $(&quot; , &quot;'&quot; , &quot;.modal-open&quot; , &quot;'&quot; , &quot;).focus();
        }
    });

    function closeModal(e) {
        $(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).hide();
        if (e.target.name == &quot; , &quot;'&quot; , &quot;frb-modal-close-button&quot; , &quot;'&quot; , &quot;) {
            frb.hide();
            frb.altSetHideCookie( &quot; , &quot;'&quot; , &quot;donate close&quot; , &quot;'&quot; , &quot;, 604800 );
            e.target.blur();
        }
        return false;
    };

    $(&quot; , &quot;'&quot; , &quot;#frb-main&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;unFixed&quot; , &quot;'&quot; , &quot;, function() {
        $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).css( &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).position().top + 40 );
    });

    if ( country == &quot;US&quot; || country == &quot;CA&quot; || country == &quot;GB&quot; || country == &quot;IE&quot; || country == &quot;AU&quot; || country == &quot;NZ&quot; ) {
        $(&quot; , &quot;'&quot; , &quot;#frb-main&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-6c&quot; , &quot;'&quot; , &quot;);
    }

    if ( frb.shouldShowBanner() ) {
        frb.initNag();
        frb.show();
    }

});

		&quot;) or . = concat(&quot;
			

/* css variables */
:root {
    --wmui-base100: #fff;
    --wmui-base90: #f8f9fa;
    --wmui-base80: #eaecf0;
    --wmui-base70: #c8ccd1;
    --wmui-base50: #a2a9b1;
    --wmui-base30: #72777d;
    --wmui-base20: #54595d;
    --wmui-base10: #202122;
    --wmui-base0: #000;
    --wmui-accent: #36c;
    --wmui-accent-light: #eaf3ff;
    --wmui-accent-dark: #2a4b8d;
    --wmui-red: #d33;
    --wmui-red-light: #fee7e6;
    --wmui-red-dark: #b32424;
    --wmui-green: #00af89;
    --wmui-green-light: #d5fdf4;
    --wmui-green-dark: #14866d;
    --wmui-yellow: #fc3;
    --wmui-yellow-light: #fef6e7;
    --wmui-yellow-dark: #ac6600;
    --frb-primary: #2e5cb8;
    --frb-primary-light: #dde4f3;
    --frb-primary-dark: #2a4b8d;
    --frb-body: var(--wmui-base0);
    --frb-link: var(--wmui-accent);
    --frb-link-hover: #447ff5;
    --frb-message-background: var(--wmui-base100);
    --frb-message: var(--wmui-base0);
    --frb-message-border: #900;
    --frb-muted: var(--wmui-base20);
    --frb-muted-hover: var(--wmui-base0);
    --frb-radio: var(--wmui-accent);
    --frb-button: var(--wmui-base90);
    --frb-button-border: var(--wmui-base50);
    --frb-button-hover: var(--wmui-accent-light);
    --frb-button-border-hover: var(--wmui-base50);
    --frb-button-focus: var(--wmui-accent-light);
    --frb-button-border-focus: var(--wmui-base50);
    --frb-button-selected: var(--frb-primary-dark);
    --frb-button-border-selected: var(--frb-primary-dark);
    --frb-submit: var(--wmui-accent);
    --frb-submit-border: var(--wmui-accent);
    --frb-submit-hover: #447ff5;
    --frb-focus: var(--wmui-accent);
    --frb-error: var(--wmui-red);
    --frb-katie-gold: #ffcc00;
}

/* Hide when editing */
.action-edit #centralNotice,
.ve-activated #centralNotice {
    display: none !important;
}

/* Fix fixed position z-index for de.wikipedia and &quot; , &quot;'&quot; , &quot;gesproken_wiki&quot; , &quot;'&quot; , &quot; element on nl.wikipedia */
.mw-body { z-index: auto; }
#siteNotice { z-index: 3; }

/* Border-Box */

.frb,
.frb *,
.frb *:before,
.frb *:after {
    -moz-box-sizing: border-box;
    -webkit-box-sizing: border-box;
    box-sizing: border-box;
}

/* Banner wide settings */

.frb input,
.frb button {
    font-size: inherit;
    font-family: inherit;
}

.frb button {
    cursor: pointer;
    border: 0;
    background: transparent;
    padding: 0;
}

.frb frb-amt,
.frb-replaced {
    white-space: nowrap;
}

@media (prefers-reduced-motion: reduce) {
    .frb,
    .frb * {
        transition-duration: 0.01ms !important;
    }
}

/* --- Main banner wrapper --- */

.frb {
    display: none;
    background-color: var(--wmui-base100);
    color: var(--frb-body);
    font-size: 16px;
    line-height: 1.1875; /*19px @16px*/
    font-family: system-ui, -apple-system,BlinkMacSystemFont, &quot;Segoe UI&quot;, Roboto, &quot;Oxygen-Sans&quot;, Ubuntu, Cantarell, Lato, &quot;Helvetica Neue&quot;, Helvetica, Arial, sans-serif;
    text-align: left; /* needed because of #siteNotice { text-align: center; } in MediaWiki */
    font-weight: normal;
    font-style: normal; /* needed for uk.wikipedia */
}

body.rtl .frb {
    text-align: right;
}

.frb-in-article {
    margin-bottom: 20px;
}

.frb-nag,
.frb-fixed {
    position: fixed;
    bottom: 0;
    left: 0;
    width: 100%;
    padding: 16px;
    background-color: var(--wmui-base100);
    border-top: 1px solid var(--wmui-base70);
    box-shadow: 0 -1px 1px rgba(0,0,0,0.1);
}

/* Avoid overlapping Vector 2022&quot; , &quot;'&quot; , &quot;s fixed width toggle */
@media ( min-width: 1400px ) {
    body.skin-vector-2022 .frb-nag,
    body.skin-vector-2022 .frb-fixed {
        left: 48px;
        right: 48px;
        width: auto;
        border-left: 1px solid var(--wmui-base70);
        border-right: 1px solid var(--wmui-base70);
    }
}

.frb-layout {
    display: grid;
    grid-template-columns: auto 330px;
    grid-template-rows: 420px auto;
    grid-template-areas:
        &quot;main sidebar&quot;
        &quot;footer sidebar&quot;;
}

@media (max-width: 959px) {
    .frb-layout {
        grid-template-rows: auto auto;
    }
}

/* --- Icon buttons --- */

.frb .frb-icon-btn {
    display: block;
    cursor: pointer;
    background-repeat: no-repeat;
    background-position: center;
    opacity: .55;
}
.frb .frb-icon-btn:hover {
    opacity: 1;
}

.frb .frb-close {
    position: absolute;
    top: 0;
    right: 0;
    width: 45px;
    height: 45px;
    background-image: url(&quot;data:image/svg+xml,%3Csvg viewBox=&quot; , &quot;'&quot; , &quot;0 0 10 10&quot; , &quot;'&quot; , &quot; xmlns=&quot; , &quot;'&quot; , &quot;http://www.w3.org/2000/svg&quot; , &quot;'&quot; , &quot;%3E%3Cg stroke=&quot; , &quot;'&quot; , &quot;%23000&quot; , &quot;'&quot; , &quot; fill=&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot; stroke-linecap=&quot; , &quot;'&quot; , &quot;round&quot; , &quot;'&quot; , &quot;%3E%3Cpath d=&quot; , &quot;'&quot; , &quot;M1 1 l8 8 M9 1 l-8 8&quot; , &quot;'&quot; , &quot;%3E%3C/path%3E%3C/g%3E%3C/svg%3E&quot;);
    background-size: 16px 16px;
}
.frb .frb-step.frb-step-1 .frb-close {
    top: -15px;
    right: -15px;
}
body.rtl .frb .frb-close {
    right: auto;
    left: 0;
}
.frb-nag-layout .frb-close {
    top: -17px;
    right: -17px;
}

.frb .frb-back {
    position: absolute;
    top: 0;
    left: 0;
    width: 45px;
    height: 45px;
    background-image: url(&quot;data:image/svg+xml,%3Csvg xmlns=&quot; , &quot;'&quot; , &quot;http://www.w3.org/2000/svg&quot; , &quot;'&quot; , &quot; viewBox=&quot; , &quot;'&quot; , &quot;0 0 20 16&quot; , &quot;'&quot; , &quot;%3E%3Cg fill=&quot; , &quot;'&quot; , &quot;none&quot; , &quot;'&quot; , &quot; fill-rule=&quot; , &quot;'&quot; , &quot;evenodd&quot; , &quot;'&quot; , &quot; transform=&quot; , &quot;'&quot; , &quot;translate(1 1)&quot; , &quot;'&quot; , &quot;%3E%3Cpath stroke=&quot; , &quot;'&quot; , &quot;%23000&quot; , &quot;'&quot; , &quot; stroke-linecap=&quot; , &quot;'&quot; , &quot;round&quot; , &quot;'&quot; , &quot; stroke-linejoin=&quot; , &quot;'&quot; , &quot;round&quot; , &quot;'&quot; , &quot; stroke-width=&quot; , &quot;'&quot; , &quot;1.778&quot; , &quot;'&quot; , &quot; d=&quot; , &quot;'&quot; , &quot;M7.181 13.285L.753 7 7.181.715&quot; , &quot;'&quot; , &quot;%3E%3C/path%3E%3Crect fill=&quot; , &quot;'&quot; , &quot;%23000&quot; , &quot;'&quot; , &quot; width=&quot; , &quot;'&quot; , &quot;18.182&quot; , &quot;'&quot; , &quot; height=&quot; , &quot;'&quot; , &quot;1.778&quot; , &quot;'&quot; , &quot; x=&quot; , &quot;'&quot; , &quot;.818&quot; , &quot;'&quot; , &quot; y=&quot; , &quot;'&quot; , &quot;6.111&quot; , &quot;'&quot; , &quot; rx=&quot; , &quot;'&quot; , &quot;.889&quot; , &quot;'&quot; , &quot;%3E%3C/rect%3E%3C/g%3E%3C/svg%3E&quot;);
    background-size: 20px 13px;
}
body.rtl .frb .frb-back {
    left: auto;
    right: 0;
    transform: rotate(180deg);
}

/* --- RML Back button --- */

.back-rml, .frb-nag .frb-rml-close-wrapper {
    display: none;
}
.frb-nag .back-rml {
    text-align: center;
    margin: 0 auto;
    padding: 2px;
    font-size: 11px;
    line-height: 1;
    text-transform: uppercase;
    cursor: pointer;
    color: var(--frb-muted);
    height: 45px;
}
.frb-nag .back-rml:hover {
    color: var(--frb-muted-hover);
}

.frb-rml-displayed .back-rml {
    display: block;
}

/* --- RML Close button --- */

.frb-rml-close-wrapper {
    text-align: center;
}

.frb-rml-close-wrapper button {
    font-size: 12px;
    color: var(--frb-muted);
    height: 45px;
}

.frb-rml-close-wrapper:hover button {
    color: var(--frb-muted-hover);
}

.frb-rml-close-icon {
    width: 10px;
    height: 10px;
    margin-bottom: -1px;
}

.frb-rml-close-icon g {
    stroke: currentColor;
}

/* -------------- Message -------------- */

.frb-message {
    grid-area: main;
    display: flex;
    flex-direction: column;
    justify-content: center;
    position: relative;
    border-radius: .75em;
    background: #308557;
    border: 6px solid #308557;
    color: var(--wmui-base100);
    font-weight: normal;
    font-size: 14px;
    line-height: 1.6; /*24px @15px*/
    z-index: 1;
    padding: 20px;
}

@media (min-width: 860px) {
    .frb-message {
        padding: 10px 20px 20px;
    }
}

@media (min-width: 960px) {
    .frb-message {
        font-size: 1.175vw;
    }
}

@media (min-width: 1200px) {
    .frb-message {
        font-size: 1.1vw;
    }
}

@media (min-width: 1400px) {
    .frb-message {
        font-size: 1vw;
    }
}

@media (min-width: 1800px) {
    .frb-message {
        font-size: 18px;
    }
}

.frb-message-icon {
    float: left;
    margin-top: 4px; /*in px since margin is consistent on all bp*/
    margin-right: 2px;
    height: 1em;
    width: 1em;
}

.frb-message-icon circle {
    fill: #FEFD34;
}

.frb-nag .frb-message {
    border: 6px solid var(--frb-message-border);
}

.frb-nag .frb-message-icon circle {
    fill: var(--frb-message-border);
}
.frb-message-icon path {
    fill: var(--wmui-base0);
}

.frb-nag .frb-message-icon path {
    fill: var(--wmui-base100);
}

.frb-nag .frb-message-icon {
    margin-top: 3px;
}

@media all and (min-width: 1300px) {
    .frb-nag .frb-message-icon {
        margin-top: 4px;
    }
}

body.rtl .frb-message-icon {
    float: right;
    margin-right: 0;
    margin-left: 4px;
}

.frb-greeting .frb-message-icon {
    float: none;
    margin-right: 0;
    margin-top: 0;
    margin-bottom: -2px;
}

.frb-message p {
    margin: 0 0 1em;
}

.frb-message p:last-child {
    margin: 0;
}

.frb-greeting {
    flex: 0 0 auto;
    max-height: 62px;
    margin-bottom: 0.5em;
    text-align: center;
    font-size: 1.75em;
    line-height: 1;
    font-weight: bold;
}

.frb-subgreeting {
    font-size: 0.6em;
    line-height: 1.6;
    font-weight: normal;
}

.frb-message-content {
    padding-bottom: 25px;
}

@media (max-width: 959px) {
    .frb-greeting {
        max-height: none;
    }
    .frb-message-content {
        font-size: 14px !important;
    }
}

@media (max-width: 860px) {
    .frb-message-content {
        padding-bottom: 55px;
    }
}


/* Nag styles */

.frb-nag {
    cursor: pointer;
}

.frb-nag-layout {
    display: grid;
    grid-template-areas: &quot;main sidebar&quot;;
    grid-template-columns: auto 360px;
}

@media (min-width: 1200px) {
    .frb-nag-layout {
        grid-template-columns: auto 440px;
    }
}

.frb-nag .frb-message {
    padding: 22px 26px;
    background: var(--frb-message-background);
    color: var(--frb-message);
}

@media (max-width: 1400px) {
    .frb-nag .frb-message {
        padding: 18px 18px;
        font-size: 16px;
    }
}

@media (max-width: 1100px) {
    .frb-nag .frb-message {
        font-size: 14px;
    }
}

.frb-nag .frb-form-wrapper {
    padding: 0 16px;
    display: flex;
    align-items: center;
    justify-content: center;
}

/* Triangle */
.frb-message::after {
    position: absolute;
    content: &quot; &quot;;
    top: 180px;
    right: -25px;
    margin: -10px 0 0 0;
    border: 10px solid transparent;
    border-left-color: #308557;
    pointer-events: none;
}
body.rtl .frb-message::after {
    right: auto;
    left: -25px;
    border-left-color: transparent;
    border-right-color: #308557;
}

.frb-nag .frb-message::after {
    top: 50%;
    border-left-color: var(--frb-message-border);
}

body.rtl .frb-nag .frb-message::after {
    border-left-color: transparent;
    border-right-color: var(--frb-message-border);
}

/* -------------- Form -------------- */

.frb-form-wrapper {
    grid-area: sidebar;
    position: relative;
    background: var(--wmui-base100);
}

.frb-form-wrapper fieldset {
    border: 0;
    margin: 0 auto;
    padding: 0 0 6px 0;
}

.frb-form-wrapper .frb-amounts {
    padding: 0;
    margin-top: 8px;
}

.frb-form-wrapper legend,
.frb-rml-form-legend {
    display: block;
    margin: 0 0 2px;
    padding: 0 4px;
    font-weight: normal;
    text-align: inherit;
    font-size: 14px;
    line-height: 1.2142857143; /*17px @14px*/
    color: var(--frb-muted);
    transition: all .25s ease-in-out;
}

.frb-form-wrapper {
    counter-reset: count;
}
.frb-numbered {
    counter-increment: count;
}
.frb-numbered::before {
    content: counter(count) &quot;. &quot;;
    position: absolute;
    left: -12px;
}
body.rtl .frb-numbered::before {
    left: auto;
    right: -12px;
}

.frb-rml-form-legend {
    padding: 0 0 2px;
}

.frb-frequency legend,
.frb-amounts legend {
    padding: 0 5px;
}

.frb-form-wrapper fieldset:first-of-type legend {
    padding-top: 0;
}

.frb-form-wrapper ul {
    display: flex;
    flex-wrap: wrap;
    width: 100%;
    margin: 0;
    padding: 0;
    list-style: none;
}

.frb-form-wrapper li {
    display: inline-flex;
    justify-content: flex-start;
    align-items: center;
    margin: 0;
}

#frb-form {
    padding-left: 25px;
    position: relative;
    overflow: hidden;
    height: 100%; /* ensure varying height steps don&quot; , &quot;'&quot; , &quot;t get cut off */
}
body.rtl #frb-form {
    padding-left: 0;
    padding-right: 25px;
}

.frb-frequency ul li {
    flex: 1 0 0;
}

.frb-amounts ul li {
    flex: 0 0 32%;
    max-width: 32%;
}

.frb-amounts ul li.frb-amt-other {
    flex: 0 0 67%;
    max-width: 67%;
}

.frb-amounts .frb-radio-label {
    white-space: nowrap;
}

/* --- Common Button Styles --- */

/* Hide radio buttons */
.frb-form-wrapper .frb-methods input[type=&quot;radio&quot;],
.frb-form-wrapper .frb-optin input[type=&quot;radio&quot;],
.frb-form-wrapper input[type=&quot;checkbox&quot;] {
    position: absolute;
    overflow: hidden;
    height: 1px;
    width: 1px;
    clip: rect(0 0 0 0);
    border: 0;
    margin: -1px;
    padding: 0;
}

/* TODO: are these frb-btn styles needed? */
.frb-btn {
    width: 100%;
    height: 48px;
    display: block;
    background-color: var(--frb-button);
    color: var(--frb-body);
    font-size: 16px;
    line-height: 1.25; /*20px @16px*/
    padding: 13px 4px 15px 4px;
    border: 1px solid var(--frb-button-border);
    border-radius: 2px;
    outline: 0;
    text-align: center;
    cursor: pointer;
    font-family: inherit;
    font-weight: 500;
    transition: all .25s ease-in-out;
}
.frb-btn:hover {
    background-color: var(--wmui-base100);
    color: #444;
    border-color: var(--wmui-base50);
}
.frb-btn:active {
    background-color: #d9d9d9;
    color: var(--wmui-base0);
    border-color: #7d8389;
}
.frb-form-wrapper input[type=radio]:checked + .frb-btn {
    background-color: var(--frb-primary-dark);
    color: var(--wmui-base100);
    border-color: #7d8389;
}

.frb-rml-link {
    display: block;
    font-size: 16px;
    line-height: 1.125; /*18px @16px*/
    color: var(--frb-link);
    margin: 16px auto 0;
    text-align: center;
    font-weight: bold;
}

.frb-rml-link:hover,
.frb-rml-link:focus {
    color: var(--frb-link-hover);
}

.frb-radio,
.frb-radio-label {
    font-size: 16px;
    line-height: 1.375; /*22px @16px*/
}

.frb-radio {
    cursor: pointer;
    margin: 0 0 0 7px;
}

.frb-radio-label {
    display: block;
    padding: 12px 7px;
    cursor: pointer;
    font-weight: bold;
    flex: 1 0 auto;
}

/* Focus styles */

/*Outline reset*/
.frb-form-wrapper input[type=radio]:focus,
.frb-radio:focus + .frb-radio-label,
#frb-amt-other-input:focus,
#frb-rml-email:focus {
    outline: 0;
}

.frb button:focus,
.frb-btn:focus,
.frb-icon-btn:focus,
.frb-btn-submit:focus,
.frb-form-wrapper input[type=radio]:focus + .frb-btn,
.frb-form-wrapper input[type=radio]:focus + #frb-amt-other-label,
#frb-amt-other-input:focus,
.frb-rml-displayed .frb-rml-form input:focus,
.frb-nag-btn:focus,
#nag-rml-btn:focus {
    outline: 0;
    border-color: var(--frb-focus) !important;
    box-shadow: inset 0 0 0 2px var(--frb-focus);
}

.frb button.frb-submit:focus {
    box-shadow: inset 0 0 0 1px var(--frb-submit-hover), inset 0 0 0 2px var(--wmui-base100);
}

.frb-rml-displayed .frb-rml-form input:focus,
.frb-rml-displayed .frb-rml-form .frb-btn-submit:focus {
    position: relative;
}

.frb-radio:enabled:focus + .frb-radio-label,
.frb-radio:enabled:hover + .frb-radio-label {
    color: var(--frb-link);
    text-decoration: underline;
}

.frb-radio:disabled + label {
    opacity: 0.4;
    cursor: default;
}

#frb-amt-other-input:focus,
#frb-amt-other-input:hover {
    box-shadow: none;
    box-shadow: 0 2px #fff, 0 3px var(--frb-link);
    color: var(--frb-link);
}

.frb-form-wrapper input[type=radio]:focus + .frb-btn,
.frb-form-wrapper input[type=radio]:focus + #frb-amt-other-label,
.frb-form-wrapper input[type=radio]:checked + .frb-btn:focus,
.frb-form-wrapper input[type=radio]:checked + #frb-amt-other-label:focus,
.frb-form-wrapper .frb-btn-submit:focus,
#nag-yes-btn:focus {
    box-shadow: inset 0 0 0 1px var(--frb-submit), inset 0 0 0 2px var(--wmui-base100);
}

.frb-btn img {
    padding: 0 4px;
    max-width: 100%;
}

.frb-methods .frb-btn {
    height: 64px;
    line-height: 1.125; /*18px @16px*/
}

.frb-methods svg {
    max-width: 100%;
    width: 64px;
}

/*Slight adaption for Paypal logo with USD string*/
.frb-methods .frb-logo-payments--paypal-usd {
    width: 85px;
    margin-bottom: -6px;
}

/* -- Credit card logos -- */

.frb-cc-logo-wrapper {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    align-items: center;
    margin: 0 auto;
    max-width: 80px;
    font-size: 0; /* Remove spacing between icons */
}

.frb-pm-cc svg {
    flex: 0 0 24px;
    max-width: 24px;
    width: 24px;
    max-height: 15px; /* height needed for IE11 */
    margin: 2px;
    display: none;
}

/* Reduce card logo spacing/sizing when there are 4 methods */
.frb-payment-options .frb-button:first-child:nth-last-child(4) .frb-cc-logo-wrapper,
.frb-payment-options .frb-button:first-child:nth-last-child(4) ~ .frb-button .frb-cc-logo-wrapper {
    width: 61px;
}

.frb-payment-options .frb-button:first-child:nth-last-child(4) .frb-cc-logo-wrapper svg,
.frb-payment-options .frb-button:first-child:nth-last-child(4) ~ .frb-button .frb-cc-logo-wrapper svg {
    width: 24px;
    height: 15px;
}

/* Countries with 3 card types */
.frb-cctypes-vma .frb-cc-logo-wrapper {
    width: 100%;
    flex-wrap: nowrap;
}
.frb-cctypes-vma svg  {
    flex: 0 0 28%;
    max-width: 28%;
    width: 28%;
    max-height: 34px;
}

.frb-cc-logo-wrapper {
    display: none;
}

.frb-cctypes-vmad .frb-cc-logo-wrapper,
.frb-cctypes-vmaj .frb-cc-logo-wrapper,
.frb-cctypes-vma  .frb-cc-logo-wrapper,
.frb-cctypes-vm   .frb-cc-logo-wrapper {
    display: flex;
}

.frb-cctypes-vmad .frb-pm-cc-label,
.frb-cctypes-vmaj .frb-pm-cc-label,
.frb-cctypes-vma  .frb-pm-cc-label,
.frb-cctypes-vm   .frb-pm-cc-label {
    display: none;
}

.frb-cctypes-vmad .frb-cc-logo-visa,
.frb-cctypes-vmad .frb-cc-logo-mastercard,
.frb-cctypes-vmad .frb-cc-logo-amex,
.frb-cctypes-vmad .frb-cc-logo-discover,

.frb-cctypes-vmaj .frb-cc-logo-visa,
.frb-cctypes-vmaj .frb-cc-logo-mastercard,
.frb-cctypes-vmaj .frb-cc-logo-amex,
.frb-cctypes-vmaj .frb-cc-logo-jcb,

.frb-cctypes-vma  .frb-cc-logo-visa,
.frb-cctypes-vma  .frb-cc-logo-mastercard,
.frb-cctypes-vma  .frb-cc-logo-amex,

.frb-cctypes-vm   .frb-cc-logo-visa,
.frb-cctypes-vm   .frb-cc-logo-mastercard {
    display: inline-block;
}

/* Submit/Continue buttons (blue background) */
.frb .frb-btn-submit {
    width: 100%;
    display: block;
    margin-top: 6px;
    padding: 8px;
    color: var(--wmui-base100);
    background-color: var(--frb-submit);
    border-color: var(--frb-submit);
    cursor: pointer;
    border: 0;
    border-radius: 2px;
    font-size: 16px;
    font-weight: bold;
    transition: all .25s ease-in-out;
    height: 45px;
}
.frb .frb-btn-submit:hover {
    background-color: var(--frb-submit-hover);
    border-color: var(--frb-submit-hover);
}
.frb .frb-btn-submit:active {
    background-color: var(--frb-primary-dark);
    border-color: var(--frb-primary-dark);
    box-shadow: none;
}

.frb-submit-txt-once { display: inline; }
.form-monthly .frb-submit-txt-once { display: none; }

.frb-submit-txt-monthly { display: none; }
.form-monthly .frb-submit-txt-monthly { display: inline; }

/* --- Other Amount --- */

/* TODO: Can we cut these down at all? */
#frb-amt-other-input::-webkit-input-placeholder {
    color: var(--frb-body);
}
#frb-amt-other-input::-moz-placeholder {
    opacity: 1;
    color: var(--frb-body);
}
#frb-amt-other-input:-ms-input-placeholder {
    color: var(--frb-body);
}
#frb-amt-other-input:-moz-placeholder {
    opacity: 1;
    color: var(--frb-body);
}

#frb-amt-other-input:focus::-webkit-input-placeholder {
    color: #666;
}
#frb-amt-other-input:focus::-moz-placeholder {
    opacity: 1;
    color: #666;
}
#frb-amt-other-input:focus:-ms-input-placeholder {
    color: #666;
}
#frb-amt-other-input:focus:-moz-placeholder {
    opacity: 1;
    color: #666;
}

#frb-amt-other-input:hover::-webkit-input-placeholder {
    color: var(--frb-link);
}
#frb-amt-other-input:hover::-moz-placeholder {
    opacity: 1;
    color: var(--frb-link);
}
#frb-amt-other-input:hover:-ms-input-placeholder {
    color: var(--frb-link);
}
#frb-amt-other-input:hover:-moz-placeholder {
    opacity: 1;
    color: var(--frb-link);
}

.frb-amt-other {
    width: 66.666666%;
}

#frb-amt-other-input {
    width: calc(100% - 33px);
    padding: 4px 2px 4px 7px;
    border: none;
    direction: ltr;
    text-align: left;
    font-size: 16px;
    font-family: inherit;
    font-weight: bold;
    color: var(--frb-body);
    box-shadow: 0 2px #fff, 0 3px var(--frb-body);
    -webkit-appearance: none;
    border-radius: 0; /* needed for iPad */
}

/* TODO: What is this for? */
@media all and (min-width: 1280px) {
    .frb-amt-other #frb-amt-other-input {
        margin: 0;
    }
}

body.rtl #frb-amt-other-input {
    text-align: right;
}

/* --- Transaction fees options --- */

/* Checkbox styles */

.frb-checkbox-label {
    position: relative;
    display: inline-block;
    margin: 13px 0 0 5px;
    width: calc(100% - 10px);
    padding-left: 26px;
    padding-top: 2px;
    cursor: pointer;
    font-size: 14px;
    line-height: 1.3571428571; /*19px @14px*/
    color: var(--frb-body);
}
body.rtl .frb-checkbox-label {
    margin: 13px 5px 0 0;
    padding-left: 0;
    padding-right: 26px;
}

/* Outer-box */
.frb-checkbox-label::before {
    position: absolute;
    content: &quot;&quot;;
    top: 3px;
    left: 0;
    display: inline-block;
    height: 17px;
    width: 17px;
    border-radius: 2px;
    border: 1px solid var(--wmui-base50);
    background-color: var(--wmui-base90);
}
body.rtl .frb-checkbox-label::before {
    left: auto;
    right: 0;
}

/* Checkmark */
.frb-checkbox-label::after {
    position: absolute;
    content: &quot;&quot;;
    top: 8px;
    left: 4px;
    display: inline-block;
    height: 5px;
    width: 9px;
    border-left: 2px solid;
    border-bottom: 2px solid;
    transform: rotate(-45deg);
    border-color: var(--wmui-base100);
}
body.rtl .frb-checkbox-label::after {
    left: auto;
    right: 4px;
}

.frb-ptf-total {
    font-weight: bold;
}
.frb-ptf-fee {
    white-space: nowrap;
}

/* Hide the checkmark by default */
.frb-checkbox + .frb-checkbox-label::after {
    content: none;
}
/* Unhide the checkmark on the checked state */
.frb-checkbox:checked + .frb-checkbox-label::after {
    content: &quot;&quot;;
}

.frb-checkbox:checked + .frb-checkbox-label:before {
    background-color: var(--wmui-accent);
    border-color: var(--wmui-accent);
}

/* Focus styles - unchecked */
.frb-checkbox:focus + .frb-checkbox-label::before {
    border-color: var(--wmui-accent);
    box-shadow: inset 0 0 0 1px var(--wmui-accent);
}

/* Focus styles - checked */
.frb-checkbox:focus:checked + .frb-checkbox-label::before {
    box-shadow: inset 0 0 0 1px var(--wmui-accent), inset 0 0 0 2px var(--wmui-base100);
}

/* Hover */
.frb-checkbox:hover + .frb-checkbox-label::before {
    background-color: var(--wmui-base80);
}

.frb-checkbox:checked:hover + .frb-checkbox-label:before {
    background-color: var(--frb-link-hover);
    border-color: var(--frb-link-hover);
}

/* --- Email opt-in --- */

.frb-form-wrapper .frb-optin {
    margin-bottom: 3px;
}

.frb-form-wrapper .frb-optin legend {
    margin-bottom: 8px;
    display: inline-block;
}

.frb-optin .frb-radio {
    margin: 6px 5px 0px 9px;
}

.frb-optin .frb-radio-label {
    float: right;
    width: calc(100% - 30px);
    white-space: normal;
    font-size: 14px;
    line-height: 1.3571428571; /*19px @14px*/
    font-weight: bold;
}

.frb-optin .frb-radio-label:hover,
.frb-optin .frb-radio:hover + .frb-radio-label {
    text-decoration: none;
}

.frb-optin-no-prompt {
    display: none;
    margin: 8px;
    padding: 4px 8px;
    border: 2px solid #900;
    border-radius: 2px;
    font-size: 14px;
    line-height: 1.2857142857; /*18px @14px*/
    font-weight: normal;
}

.frb-optin-no-prompt.is-positive {
    border-color: var(--wmui-green-dark);
    font-weight: bold;
}

.frb-optin-no-prompt__yes {
    display: none;
}

.frb-optin-no-prompt__no {
    display: block;
}

.frb-optin-no-prompt.is-positive .frb-optin-no-prompt__yes {
    display: block;
}

.frb-optin-no-prompt.is-positive .frb-optin-no-prompt__no {
    display: none;
}

.frb-optin-smallprint {
    padding: 8px 8px 0 8px;
}

/* --- Payment method Buttons --- */

/* Fade methods which aren&quot; , &quot;'&quot; , &quot;t monthly capable when monthly option is selected */
.form-monthly .no-monthly {
    opacity: 0.4 !important;
}

.form-monthly .no-monthly .frb-label {
    cursor: default;
}

.frb-form-wrapper .frb-methods {
    margin-top: 12px;
    padding-bottom: 0;
}

/* --- Footer / Small Print --- */
.frb-smallprint {
    font-size: 12px;
    line-height: 1.5; /*18px @12px*/
    color: var(--frb-muted);
    font-weight: normal;
}
.frb-smallprint a {
    color: var(--frb-muted);
    text-decoration: underline;
}
.frb-smallprint a:hover {
    color: var(--frb-muted-hover);
}

.frb-footer {
    grid-area: footer;
    padding-top: 8px;
    display: flex;
}

.frb-footer-message {
    flex: 1 1 100%;
    max-width: calc(100% - 160px);
    display: inline-block;
    padding-right: 45px;
}

.frb-footer-button {
    display: inline-flex;
    align-items: flex-start;
    flex: 0 0 160px;
    max-width: 160px;
    width: 160px;
    justify-content: flex-end;
    margin-top: 20px;
}

.frb-footer-button .frb-already-donated-button {
    font-size: 14px;
    font-weight: bold;
    display: inline-block;
    text-decoration: none;
    color: var(--frb-muted);
}
.frb-footer-button .frb-already-donated-button:hover {
    color: var(--wmui-base0);
}

@media (max-width: 1160px) {
    .frb-smallprint {
        font-size: 10px;
    }
}

/* --- Disable I already donated --- */
.frb.frb-iad-disabled .frb-iad {
    display: none;
}

/* --- Show and Hiding (Minimize and Maximize) --- */

.frb-nag-btns {
    display: flex;
    flex: 1 0 0;
}

.frb.frb-rml-displayed .frb-nag-btns {
    display: none;
}

button.frb-nag-btn {
    flex: 1 0 0;
    margin: 0 8px;
    padding: 6px;
    min-height: 48px;
    background-color: white;
    border: 1px solid var(--frb-button-border);
    border-radius: 2px;
    outline: 0;
    color: var(--frb-body);
    font-weight: normal;
    font-size: 13px;
    line-height: 1.3;
    transition: all 100ms;
    cursor: pointer;
}

@media all and (min-width: 1100px) {
    button.frb-nag-btn {
        font-size: 14px;
    }
}

@media all and (min-width: 1200px) {
    button.frb-nag-btn {
        font-size: 16px;
    }
}

button.frb-nag-btn:hover {
    transform: scale(1.043478);
}

#nag-yes-btn {
    font-weight: bold;
    background-color: var(--frb-submit);
    border: 1px solid var(--frb-submit);
    color: white;
}

/* --- Maybe later --- */

.frb-rml-disabled .frb-rml,
.frb-rml-disabled .frb-rml-link,
.frb-rml-disabled #nag-rml-btn {
    display: none;
}

#frb-main .frb-rml {
    position: absolute;
    width: 100%;
}

.frb-rml-form-wrapper {
    display: none;
    position: absolute;
    margin-top: 12px;
    margin-left: 12px;
    width: calc(100% - 12px);
    padding: 16px 16px 0 16px;
    text-align: left;
    background: var(--wmui-base100);
    border: 1px solid var(--wmui-base70);
    border-radius: 2px;
    box-shadow: 0 2px 2px 0 rgba(0, 0, 0, 0.15);
    z-index: 10;
}

.frb-rml-form-wrapper:after,
.frb-rml-form-wrapper:before {
    bottom: 100%;
    left: 50%;
    border: solid transparent;
    content: &quot; &quot;;
    height: 0;
    width: 0;
    position: absolute;
    pointer-events: none;
}

.frb-rml-form-wrapper:after {
    border-bottom-color: var(--wmui-base100);
    border-width: 10px;
    margin-left: -10px;
}

.frb-rml-form-wrapper:before {
    border-bottom-color: var(--wmui-base70);
    border-width: 11px;
    margin-left: -11px;
}


/* styles for bottom fixed banner */
.frb-fixed .frb-rml-form-wrapper:after,
.frb-fixed .frb-rml-form-wrapper:before {
    top: 100%;
    bottom: auto;
}

.frb-fixed .frb-rml-form-wrapper:after {
    border-top-color: var(--wmui-base100);
    border-bottom-color: transparent;
}

.frb-fixed .frb-rml-form-wrapper:before {
    border-top-color: var(--wmui-base70);
    border-bottom-color: transparent;
}

.frb-rml-form input {
    width: 100%;
    padding: 6px 8px 7px;
    border: 1px solid var(--wmui-base50);
    border-radius: 2px;
    color: var(--wmui-base0);
    font-size: 14px;
    height: 45px;
}

#frb-rml-email.frb-haserror {
    border-color: var(--frb-error) !important;
    box-shadow: inset 0 0 0 1px var(--frb-error) !important;
}

.frb-error-invalidemail {
    margin: 2px 0 0 !important;
}

.frb-rml-ty {
    text-align: left;
    font-size: 14px;
}

.frb-rml-displayed .frb-rml-form-wrapper {
    position: relative;
    z-index: 10;
    display: table !important;
    margin: -4px auto 0;
    padding: 0 12px;
    width: 100%;
    max-width: 340px;
    background: transparent;
    border: none;
    box-shadow: 0 0 0 0 rgba(0, 0, 0, 0);
}

.frb-rml-displayed .frb-rml-form-wrapper:after,
.frb-rml-displayed .frb-rml-form-wrapper:before {
    display: none !important;
}

.frb-rml-displayed .frb-rml {
    display: block !important;
    margin-top: 0;
}

.frb-rml-displayed .frb-rml-form legend {
   font-size: 12px;
   line-height: 1; /*12px @12px*/
   padding-bottom: 4px;
}

@media all and (min-width: 1200px) {
    .frb-rml-displayed .frb-rml-form legend {
       font-size: 14px;
       line-height: 1.2142857143; /*17px @14px*/
    }
}

.frb-rml-displayed .frb-rml-form input {
    display: inline;
    vertical-align: middle;
    width: 200px;
    height: 45px;
    padding: 7px 8px;
    margin: 0;
    border: 1px solid var(--wmui-base50);
    border-radius: 2px;
    color: var(--frb-body);
    direction: ltr;
    line-height: 1;
}

.frb-rml-displayed .frb-rml-form .frb-btn-submit {
    display: inline;
    vertical-align: middle;
    width: auto;
    height: 45px;
    margin-top: 0;
    margin-left: 2px;
    line-height: 1;
    padding: 9px 14px;
    font-size: 14px;
    border-radius: 2px;
}

.frb-prevent-page-jump {
    display: none;
}

/* -- Submit/&quot;Donate now&quot; button -- */

.frb .frb-submit {
    height: 52px;
    display: inline-block;
    cursor: default;
    margin: 4px 5px 0;
    padding: 5px 6px;
    width: calc(100% - 9px);
    background-color: var(--wmui-base100);
    border: 1px solid var(--wmui-base70);
    border-radius: 2px;
    color: rgba(84,89,93,0.2);
    font-weight: bold;
    transition: background-color 0.5s ease;
    word-break: keep-all; /* T259535 */
    line-height: 1.3;
}

.frb .frb-submit.active {
    background-color: var(--frb-submit);
    border-color: var(--frb-submit);
    color: var(--wmui-base100);
    cursor: pointer;
    opacity: 1;
}

.frb .frb-submit.active:hover {
    background-color: var(--frb-submit-hover);
    border-color: var(--frb-submit-hover);
}

.frb-submit-amount {
    display: none;
}
.frb-submit-label-monthly {
    display: none;
}
.frb-submit-label-now {
    display: inline;
}
.form-monthly .frb-submit-label-monthly {
    display: inline;
}
.form-monthly .frb-submit-label-now {
    display: none;
}
.frb-payment-options {
    display: flex;
    flex-wrap: wrap;
    width: 100%;
}

.frb-payment-options .frb-button {
    min-width: 33%;
}

/* For 4 payment options, one row */
.frb-payment-options .frb-button:first-child:nth-last-child(4),
.frb-payment-options .frb-button:first-child:nth-last-child(4) ~ .frb-button {
    min-width: 25%;
}

/* If there are 5 buttons, make the first one of the wider ones */
.frb-payment-options .frb-button:first-child:nth-last-child(5) {
    min-width: 50%;
}

.frb-methods .frb-button,
.frb-optin .frb-button {
    flex: 1 0 0;
    padding: 5px;
}

.frb-label {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 52px;
    padding: 0 4px;
    color: var(--frb-link);
    background-color: var(--frb-button);
    border: 1px solid var(--frb-button-border);
    border-radius: 2px;
    text-align: center;
    font-weight: bold;
    transition: all 0.5s ease;
    cursor: pointer;
}

.frb-pm-cc .frb-label {
    padding: 0 2px;
}

.frb-label:hover,
.frb-rml-email:hover {
    background-color: var(--frb-button-hover);
}

.form-monthly .no-monthly .frb-label:hover {
    background-color: var(--frb-button-hover);
}

.frb-radio:checked + .frb-label {
    background-color: var(--frb-button-selected);
    border-color: var(--frb-button-border-selected);
    color: var(--wmui-base100);
}

.frb-radio:focus + .frb-label,
#frb-rml-email:focus {
    box-shadow: inset 0 0 0 1px var(--frb-radio);
    border-color: var(--frb-radio);
}

.frb-radio:focus:checked + .frb-label {
    box-shadow: inset 0 0 0 1px var(--frb-button-selected), inset 0 0 0 2px var(--wmui-base100);
}

.frb-radio:checked + .frb-label .frb-logo-payments--paypal path,
.frb-radio:checked + .frb-label .frb-logo-payments--paypal-usd path,
.frb-radio:checked + .frb-label .frb-logo-payments--amazon path,
.frb-radio:checked + .frb-label .frb-logo-payments--applepay path,
.frb-radio:checked + .frb-label .frb-logo-payments--venmo path {
    fill: var(--wmui-base100);
}

/* Error messages */
.frb-error {
    display: none;
    margin: 5px 0 5px 5px;
    font-size: 14px;
    font-weight: 500;
    line-height: 1.3;
    color: var(--frb-error);
}

.frb-form-wrapper fieldset.frb-haserror .error-highlight,
.frb-form-wrapper fieldset.frb-haserror legend::before {
    color: var(--frb-error);
    font-weight: bold;
}

/* STEP 2 UPSELL*/

.frb-upsell,
.frb-step-monthly-diff-amt .frb-amt-monthly {
    transition: background-color 0.5s ease;
    padding: 10px 4px;
    text-align: center;
}

.frb-step-monthly-diff-amt .frb-amt-monthly {
    display: block;
    padding: 0 4px 10px 4px;
}

.frb-upsell-cta,
.frb-upsell-ty {
    font-size: 17px;
    line-height: 1.3;
    font-weight: bold;
    text-align: center;
}

.frb-upsell-color,
.frb-step-monthly-diff-amt .frb-amt-monthly label {
    display: block;
    font-size: 15px;
    line-height: 1.3;
    font-weight: normal;
    margin: .5em 0;
}

.frb .frb-monthly-diff-amt-link {
    font-size: 15px;
    line-height: 1.3;
    color: var(--frb-link);
    margin: 8px 2px;
    padding: 12px 10%;
    text-align: center;
    cursor: pointer;
    font-weight: bold;
}

#frb-amt-monthly-other-input {
    position: relative;
    text-align: center;
    font-size: 18px;
    height: 45px;
}

/* Steps */
#frb-form .frb-step {
    position: absolute;
    top: 0;
    padding-top: 24px;
    width: 300px;
    margin-left: 100%; /* Start hidden */
    visibility: hidden; /* Prevent tabbing to inputs */
}
body.rtl #frb-form .frb-step {
    margin-left: 0;
    margin-right: 100%;
}

#frb-form .frb-step-1 {
    position: relative;
    margin-left: 0;
    padding-top: 0;
    visibility: visible;
}
body.rtl #frb-form .frb-step-1 {
    margin-right: 0;
}
#frb-form .frb-step-optin,
#frb-form .frb-step-upsell {
    padding-top: 45px;
}

/*
    Already Donated Modal
*/
.modal {
    display: none;
    position: absolute;
    z-index: 999;
    background: rgba(255,255,255,0.95);
    min-width: 100%;
    min-height: 100%;
    top: 0;
    left: 0;
}
.modal-container {
    width: 100%;
    max-width: 600px;
    margin: 0 auto;
    box-shadow: 0 0 10px rgba(0,0,0,0.15);
    color: #000;
    background-color: #FFF;
    padding: 40px;
    box-sizing: border-box;
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%,-50%);
}
.modal-close-x {
    border: none;
    background: none;
    margin: 0;
    padding: 0;
    cursor: pointer;
    position: absolute;
    top: 24px;
    left: 24px;
    width: 22px;
    height: 18px;
    opacity: 0.5;
}

.modal-title-text {
    font-size: 17px;
    color: #262626;
    font-weight: bold;
    display: inline-block;
}
.modal-text {
    font-size: 15px;
    margin: 20px auto 0;
    line-height: 1.44;
    max-width: 530px;
}
.frb-modal-buttons {
    display: flex;
    justify-content: center;
    margin: 48px auto 0;
}
.modal-close-button {
    margin: 0 8px;
    padding: 10px 15px;
    display: block;
    border-radius: 2px;
    border: solid 1px #a2a9b1;
    background-color: #f8f9fa;
    width: 165px;
    font-family: Arial;
    font-size: 16px;
    font-weight: bold;
    color: #007de2;
    cursor: pointer;
}
.modal-close-button:hover{
    background-color: #eaf3ff;
}

/*
    Responsive
*/
@media (max-width: 1200px) {
    .frb-footer {
        display: block;
    }
    .frb-footer-message {
        display: block;
        padding-right: 0;
        max-width: 100%;
    }
    .frb-footer-button {
        display: block;
        margin: 16px auto 0;
        width: auto;
        text-align: center;
    }
    .frb-already-donated-text {
        font-size: 14px;
    }

}


/* wmf identity */

.frb-wmf-identity {
    position: absolute;
    bottom: 0;
    right: 50%;
    width: 100%;
    max-width: 100%;
    clear: both;
    color: var(--wmui-base100);
    display: flex;
    justify-content: center;
    transform: translateX(50%);
}

.frb-wmf-identity-logo {
    max-width: 100px;
    flex: 0 0 100px;
    display: flex;
    align-items: center;
    margin: 0;
}

.frb-wmf-identity-logo img {
    width: 100%;
    max-width: 100%;
    filter: invert(1);
}

@media (max-width: 859px) {
    .frb-wmf-identity {
        flex-direction: column;
        align-items: center;
        justify-content: flex-end;
    }
    .frb-wmf-identity-logo {
        flex: 0 0 auto;
    }
}

.frb-wmf-identity-message {
    flex: 0 1 auto;
    font-size: 14px;
    display: flex;
    align-items: center;
    padding-left: 20px;
    color: var(--wmui-base100);
}

.frb-wmf-identity-message p {
    margin: 0;
    font-size: 13px;
}

/* screen reader visibility */
.sr-only {
    border: 0 !important;
    clip: rect(1px, 1px, 1px, 1px) !important;
    -webkit-clip-path: inset(50%) !important;
        clip-path: inset(50%) !important;
    height: 1px !important;
    margin: -1px !important;
    overflow: hidden !important;
    padding: 0 !important;
    position: absolute !important;
    width: 1px !important;
    white-space: nowrap !important;
}

.frb-submit-txt-donate { display: none; }
.frb-submit-txt-once {display: inline; }
.frb-submit-txt-monthly { display: none; }
.form-monthly .frb-submit-txt-once { display: none; }
.form-monthly .frb-submit-txt-monthly { display: inline; }

.frb-icon-heart path {
    fill: #900;
}






    

        
            
        

        
            Thank you, dear donor!
        

        
            Your generosity helps keep Wikipedia and its sister sites thriving. Select &quot;hide appeals&quot; to suppress fundraising messages in this browser for a week, or go back to the appeal if you&quot; , &quot;'&quot; , &quot;re still interested in donating.
        

        
            Hide appeals for a week
            Back to appeal
        

    




    

        
            
                
                Wikipedia still can&quot; , &quot;'&quot; , &quot;t be sold.
                December 18: An important update for readers in the United States
            
            
                Please don&quot; , &quot;'&quot; , &quot;t close this, it’s just a 1-minute read. We&quot; , &quot;'&quot; , &quot;re sorry to interrupt, but it&quot; , &quot;'&quot; , &quot;s Monday, December 18, and it will soon be too late to help us in our year-end fundraiser. We ask you to reflect on the number of times you visited Wikipedia this year and if you&quot; , &quot;'&quot; , &quot;re able to give $2.75 to the Wikimedia Foundation. If everyone reading this gave just $2.75, we&quot; , &quot;'&quot; , &quot;d hit our goal in a few hours.

                In the age of AI, access to verifiable facts is crucial. Wikipedia matters more than ever as a reliable source for emerging technologies, and you. Your gift supports how readers use Wikipedia now, and how revolutionary new systems will utilize it tomorrow.

                Reflect on Wikipedia&quot; , &quot;'&quot; , &quot;s usefulness in your life, and if the knowledge you gained was valuable, please give $2.75. Every contribution helps: every edit, every gift counts.
            

            
               
                   
               
               
                   Proud host of Wikipedia and its sister sites
               
            
        

        

            

                

                    

                    
                        
                            How often would you like to donate?
                        
                        
                            
                                
                                One time
                            
                            
                                
                                Give monthly
                            
                        
                    

                    
                        
                            Please select an amount (USD)
                            The average donation in the United States is around $13.
                        
                        
                            
                                
                                $2.75
                            
                            
                                
                                $10
                            
                            
                                
                                $15
                            
                            
                                
                                $25
                            
                            
                                
                                $50
                            
                            
                                
                                $75
                            
                            
                                
                                $100
                            
                            
                                Other amount
                                
                                
                                Other
                            
                        
                        
                            
                            I&quot; , &quot;'&quot; , &quot;ll generously add a little to cover the transaction fees so you can keep 100% of my donation.
                        
                    

                    
                        
                            Please select a payment method
                        
                        

                            

                            
                                
                                
                                    Credit/Debit Card
                                    
                                        Visa

                                        MasterCard

                                        Amex

                                        JCB

                                        Discover
                                    
                                
                            

                            

                            

                            
                                
                                
                                    PayPal
                                
                            

                            
                                
                                
                                    
                                        Venmo
                                        
                                    
                                
                            

                            

                            
                                
                                
                                    
                                        Google Pay
                                        
                                    
                                
                            

                            

                        
                    

                    
                        
                            
                                Continue
                                
                                    Donate  one time
                                    Donate  monthly
                                
                            
                        
                    

                    Please select an amount (minimum $1)
                    We cannot accept donations greater than 25000 USD through our website. Please contact our major gifts staff at benefactors@wikimedia.org.
                    Please select a payment method

                    
                        Maybe later
                    

                

                

                    

                    
                        
                            Can we follow up and let you know if we need your help again? The support and advice we get from donors in the United States is priceless, but many donors don&quot; , &quot;'&quot; , &quot;t let us stay in touch. Will you commit today, this Monday, to staying in touch with the Wikimedia Foundation?
                        
                        
                            
                                
                                Yes
                            
                            
                                
                                No
                            
                        
                        
                            Sorry to hear that. We don&quot; , &quot;'&quot; , &quot;t email often; would you consider changing your mind?
                            Thanks for changing your mind! We’ll respect your inbox.
                        
                        
                            Your information is handled in accordance with our donor privacy policy, and each email you receive will include easy unsubscribe options.
                        
                    

                    
                        Continue
                    

                    Please select an email option

                

                

                    

                    
                        
                            
                            Almost done: Please, make it  monthly.
                        Monthly support is the best way to ensure that Wikipedia keeps thriving.
                    

                    

                        
                            No thanks! I&quot; , &quot;'&quot; , &quot;ll make a one-time donation of 
                        

                        
                            Yes, I&quot; , &quot;'&quot; , &quot;ll donate  each month
                        

                    

                    Yes, I&quot; , &quot;'&quot; , &quot;ll donate monthly, but for a different amount

                

                

                    

                    Thank you for your support!
                    
                        Enter your monthly donation amount
                        
                    
                    Please select an amount (minimum $1)
                    We cannot accept donations greater than 25000 USD through our website. Please contact our major gifts staff at benefactors@wikimedia.org.
                    
                        Donate  monthly
                    

                

            

            

        

                
                    
                        Thank you! We will send you a reminder email.
                    
                    
                        
                        
                        
                        
                        
                        
                        
                        

                        Send me an email reminder
                        
                        Submit

                        
                            Please enter a valid email address i.e. name@domain.com
                        

                    
                    
                        ← Back
                    

                    
                        
                            Close
                            
                                
                                    
                                
                            
                        
                    
                
            

        
            
Problems donating? |
Other ways to give |
Frequently asked questions |
We never sell your information. By submitting, you are agreeing to our donor privacy policy and to sharing your information with the Wikimedia Foundation and its service providers in the U.S. and elsewhere.
We never sell your information. By submitting, you are agreeing to our donor privacy policy. The Wikimedia Foundation is a nonprofit, tax-exempt organization.
We never sell your information. By submitting, you are agreeing to our donor privacy policy and to sharing your information with the Wikimedia Foundation and its service providers in the U.S. and elsewhere. The Wikimedia Foundation is a nonprofit, tax-exempt organization.
If you make a recurring donation, you will be debited by the Wikimedia Foundation until you notify us to stop. We’ll send you an email which will include a link to easy cancellation instructions.

            
                
                    I already donated
                
            
        

    




    
        
            
                
                Sorry to interrupt, but time will soon run out for you to give in 2023. Please, donate $2.75.
            
        
        
            
            
                No, but maybe later when I have more time
                Yes, I&quot; , &quot;'&quot; , &quot;ll donate $2.75
            
        
    



var frb = frb || {};

frb.HIDE_DURATION_CLOSE = 3600; // 1 hour
frb.HIDE_DURATION_RML = 604800; // 1 week

frb.show = function() {
    $(&quot; , &quot;'&quot; , &quot;.frb-in-article&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;display&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;block&quot; , &quot;'&quot; , &quot;);
};

frb.hide = function() {
    /* Hide the banner, and remove related event handlers */
    $(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;).hide();
    $(&quot; , &quot;'&quot; , &quot;.frb-prevent-page-jump&quot; , &quot;'&quot; , &quot;).hide();
    $(window).off(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;);
    $(&quot; , &quot;'&quot; , &quot;#toc a&quot; , &quot;'&quot; , &quot;).off(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;);
};

/**
 * Show the given step
 * Literally just move into place and make it visible,
 * there may be other stuff needed to prepare it.
 *
 * @param  {string} step - name of step
 */
frb.showStep = function( step ) {

    var d = frb.reduceMotion ? 0 : 300, // animation duration in ms
        posPrev, posNext, posCrnt;

    if ( $(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).hasClass(&quot; , &quot;'&quot; , &quot;rtl&quot; , &quot;'&quot; , &quot;) ) {
        posPrev = { &quot; , &quot;'&quot; , &quot;margin-right&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;-110%&quot; , &quot;'&quot; , &quot; };
        posNext = { &quot; , &quot;'&quot; , &quot;margin-right&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot; };
        posCrnt = { &quot; , &quot;'&quot; , &quot;margin-right&quot; , &quot;'&quot; , &quot;: 0 };
    } else {
        posPrev = { &quot; , &quot;'&quot; , &quot;margin-left&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;-110%&quot; , &quot;'&quot; , &quot; };
        posNext = { &quot; , &quot;'&quot; , &quot;margin-left&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;100%&quot; , &quot;'&quot; , &quot; };
        posCrnt = { &quot; , &quot;'&quot; , &quot;margin-left&quot; , &quot;'&quot; , &quot;: 0 };
    }

    function movePrev( $el ) {
        $el.animate( posPrev, d, function() {
            $(this).css({ &quot; , &quot;'&quot; , &quot;visibility&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot; });
        });
    }

    function moveNext( $el ) {
        $el.animate( posNext, d, function() {
            $(this).css({ &quot; , &quot;'&quot; , &quot;visibility&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot; });
        });
    }

    function moveCrnt( $el ) {
        $el.css({ &quot; , &quot;'&quot; , &quot;visibility&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;visible&quot; , &quot;'&quot; , &quot; }).animate( posCrnt, d );
    }

    if ( step === &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot; ) {
        moveCrnt( $(&quot; , &quot;'&quot; , &quot;.frb-step-1&quot; , &quot;'&quot; , &quot;) );
        moveNext( $(&quot; , &quot;'&quot; , &quot;.frb-step-optin, .frb-step-upsell, .frb-step-monthly-diff-amt&quot; , &quot;'&quot; , &quot;) );
    } else if ( step === &quot; , &quot;'&quot; , &quot;optin&quot; , &quot;'&quot; , &quot; ) {
        movePrev( $(&quot; , &quot;'&quot; , &quot;.frb-step-1&quot; , &quot;'&quot; , &quot;) );
        moveCrnt( $(&quot; , &quot;'&quot; , &quot;.frb-step-optin&quot; , &quot;'&quot; , &quot;) );
        moveNext( $(&quot; , &quot;'&quot; , &quot;.frb-step-upsell, .frb-step-monthly-diff-amt&quot; , &quot;'&quot; , &quot;) );
    } else if ( step === &quot; , &quot;'&quot; , &quot;upsell&quot; , &quot;'&quot; , &quot; ) {
        movePrev( $(&quot; , &quot;'&quot; , &quot;.frb-step-1, .frb-step-optin&quot; , &quot;'&quot; , &quot;) );
        moveCrnt( $(&quot; , &quot;'&quot; , &quot;.frb-step-upsell&quot; , &quot;'&quot; , &quot;) );
        moveNext( $(&quot; , &quot;'&quot; , &quot;.frb-step-monthly-diff-amt&quot; , &quot;'&quot; , &quot;) );
    } else if ( step === &quot; , &quot;'&quot; , &quot;monthly-diff-amt&quot; , &quot;'&quot; , &quot; ) {
        movePrev( $(&quot; , &quot;'&quot; , &quot;.frb-step-1, .frb-step-optin, .frb-step-upsell&quot; , &quot;'&quot; , &quot;) );
        moveCrnt( $(&quot; , &quot;'&quot; , &quot;.frb-step-monthly-diff-amt&quot; , &quot;'&quot; , &quot;) );
    } else {
        // console.log( &quot; , &quot;'&quot; , &quot;Invalid step: &quot; , &quot;'&quot; , &quot; + step );
    }

};

frb.toggleMonthly = function( monthly ) {
    if( monthly.type === &quot; , &quot;'&quot; , &quot;checkbox&quot; , &quot;'&quot; , &quot; ){
        monthly = monthly.checked;
    }
    if ( monthly ) {
        $(&quot; , &quot;'&quot; , &quot;#frb-frequency-monthly&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
        $(&quot; , &quot;'&quot; , &quot;#frb-monthly-checkbox&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
        $(&quot; , &quot;'&quot; , &quot;#frb-form&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;form-monthly&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.no-monthly input[type=radio]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
        $(&quot; , &quot;'&quot; , &quot;.no-monthly&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
        $(&quot; , &quot;'&quot; , &quot;#frb-form&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;form-monthly&quot; , &quot;'&quot; , &quot;);
        if( $( &quot; , &quot;'&quot; , &quot;.form-monthly .no-monthly input[type=radio]&quot; , &quot;'&quot; , &quot; ).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;) ) {
            $(&quot; , &quot;'&quot; , &quot;.form-monthly .no-monthly input[type=radio]&quot; , &quot;'&quot; , &quot;).removeAttr(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;);
            frb.setMethod({});
        }
        $(&quot; , &quot;'&quot; , &quot;.frb-cta-label-monthly&quot; , &quot;'&quot; , &quot;).show();
    } else {
        $(&quot; , &quot;'&quot; , &quot;#frb-frequency-onetime&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, true);
        $(&quot; , &quot;'&quot; , &quot;#frb-monthly-checkbox&quot; , &quot;'&quot; , &quot;).prop(&quot;checked&quot;, false);
        $(&quot; , &quot;'&quot; , &quot;#frb-form&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;form-monthly&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.no-monthly input[type=radio]&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
        $(&quot; , &quot;'&quot; , &quot;.frb-cta-label-monthly&quot; , &quot;'&quot; , &quot;).hide();
    }
};

frb.rml = {

    post: function() {
        /* Create the iframe for the form and use it as the form&quot; , &quot;'&quot; , &quot;s target */
        var frameName = &quot; , &quot;'&quot; , &quot;remindFrame&quot; , &quot;'&quot; , &quot;;
        var $form = $(&quot; , &quot;'&quot; , &quot;#frb-rml-form&quot; , &quot;'&quot; , &quot;);
        if ( $(&quot;iframe[name=&quot; + frameName + &quot;]&quot;).length === 0 ) {
            var $iframe = $(&quot; , &quot;'&quot; , &quot;&lt;iframe style=&quot;display: none;&quot; name=&quot;&quot; , &quot;'&quot; , &quot; + frameName + &quot; , &quot;'&quot; , &quot;&quot;>&lt;/iframe>&quot; , &quot;'&quot; , &quot;);
            $form.attr(&quot; , &quot;'&quot; , &quot;target&quot; , &quot;'&quot; , &quot;, $iframe.attr(&quot; , &quot;'&quot; , &quot;name&quot; , &quot;'&quot; , &quot;));
            $form.after($iframe);
        }
        $form[0].submit();
    },

    getCurrentDate: function() {
        /* Get current date in correct format for Silverpop */
        var today = new Date();
        var dd = today.getDate();
        var mm = today.getMonth()+1; // January is 0!
        var yyyy = today.getFullYear();

        if( dd &lt; 10 ) {
            dd = &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot; + dd;
        }
        if( mm &lt; 10 ) {
            mm = &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot; + mm;
        }

        return mm+&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;+dd+&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;+yyyy;
    },

    init: function() {
        /* Prep the reminder form */

        var form = document.getElementById(&quot; , &quot;'&quot; , &quot;frb-rml-form&quot; , &quot;'&quot; , &quot;);

        form.rml_country.value    = mw.centralNotice.data.country;
        form.rml_language.value   = mw.centralNotice.data.uselang;
        form.rml_submitDate.value = frb.rml.getCurrentDate();
        form.rml_segment.value    = Math.floor((Math.random() * 100) + 1);

        $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).click(function() {
            form.rml_source.value = &quot; , &quot;'&quot; , &quot;B2324_121810_en6C_dsk_p1_lg_txt_169C&quot; , &quot;'&quot; , &quot;;
            if ($(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;).hasClass(&quot; , &quot;'&quot; , &quot;frb-fixed&quot; , &quot;'&quot; , &quot;)) {
                $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).css( &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).position().top - 195 );
            }
            else {
                $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).css( &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).position().top + 40 );
            }
            $(&quot; , &quot;'&quot; , &quot;.frb-rml-form-wrapper&quot; , &quot;'&quot; , &quot;).toggle();
            $(&quot; , &quot;'&quot; , &quot;#frb-rml-email&quot; , &quot;'&quot; , &quot;).focus();
            $(&quot; , &quot;'&quot; , &quot;.frb-rml-close-wrapper&quot; , &quot;'&quot; , &quot;).show();
            return false;
        });

        // Move the rml popup if it&quot; , &quot;'&quot; , &quot;s open when PTF or error gets shown
        $(&quot; , &quot;'&quot; , &quot;.frb-amounts input&quot; , &quot;'&quot; , &quot;).on( &quot; , &quot;'&quot; , &quot;click input change&quot; , &quot;'&quot; , &quot;, function() {
            $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).css( &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).position().top + 40 );
            setTimeout( function() {
                $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).css( &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).position().top + 40 );
            }, 400 );
        });

        $(&quot; , &quot;'&quot; , &quot;#frb-rml-submit&quot; , &quot;'&quot; , &quot;).click(function() {
            if ( mw.util.validateEmail( form.Email.value ) ) {
                frb.rml.post();
                $(&quot; , &quot;'&quot; , &quot;.frb-rml-form-wrapper form&quot; , &quot;'&quot; , &quot;).hide();
                $(&quot; , &quot;'&quot; , &quot;.frb-rml-ty&quot; , &quot;'&quot; , &quot;).show();
                window.setTimeout( function() {
                    frb.hide();
                }, 2500);
                frb.altSetHideCookie( &quot; , &quot;'&quot; , &quot;close&quot; , &quot;'&quot; , &quot;, frb.HIDE_DURATION_RML );
                return false;
            } else {
                $(&quot; , &quot;'&quot; , &quot;#frb-rml-email&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;).focus();
                $(&quot; , &quot;'&quot; , &quot;.frb-error-invalidemail&quot; , &quot;'&quot; , &quot;).show();
                return false;
            }
        });
    }

};

frb.showSidebarTooltip = function () {
    mw.loader.using( [ &quot; , &quot;'&quot; , &quot;oojs-ui-core&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mediawiki.Uri&quot; , &quot;'&quot; , &quot; ] ).done( function () {

        let menuPinned;
        if ( mw.config.get(&quot; , &quot;'&quot; , &quot;skin&quot; , &quot;'&quot; , &quot;) === &quot; , &quot;'&quot; , &quot;vector-2022&quot; , &quot;'&quot; , &quot; ) {
            menuPinned = $(&quot; , &quot;'&quot; , &quot;#vector-main-menu-pinned-container > #vector-main-menu&quot; , &quot;'&quot; , &quot;).length > 0;
        } else {
            menuPinned = true; // sidebar always visible on Legacy Vector
        }

        let $donateLink = $( &quot; , &quot;'&quot; , &quot;#n-sitesupport a&quot; , &quot;'&quot; , &quot; );
        $donateLink.attr( &quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, ( i, oldUrl ) => {
            let uri = new mw.Uri( oldUrl );
            uri.query.utm_source = &quot; , &quot;'&quot; , &quot;tooltipOnBannerClose&quot; , &quot;'&quot; , &quot;;
            return uri.toString();
        });

        let popup = new OO.ui.PopupWidget( {
            $content: $( &quot; , &quot;'&quot; , &quot;&lt;p>You can donate at any time from this menu.&lt;/p>&quot; , &quot;'&quot; , &quot; ),
            padded: true,
            autoclose: true,
            align: &quot; , &quot;'&quot; , &quot;forwards&quot; , &quot;'&quot; , &quot;,
            autoFlip: false,
            $floatableContainer: menuPinned ? $donateLink : $( &quot; , &quot;'&quot; , &quot;#vector-main-menu-dropdown&quot; , &quot;'&quot; , &quot; ),
            position: menuPinned ? &quot; , &quot;'&quot; , &quot;after&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;below&quot; , &quot;'&quot; , &quot;,
        } );

        popup.$element.css(&quot; , &quot;'&quot; , &quot;z-index&quot; , &quot;'&quot; , &quot;, 5); // Fix so it shows above header
        $( document.body ).append( popup.$element );
        popup.toggle( true );

        setTimeout( () => {
            popup.$element.fadeOut( frb.fadeDuration );
        }, 5000 );
    } );
};

/* jshint maxerr: 600 */
frb.amounts = frb.amounts || {};

// Hard minimum amounts that can be given
// From https://github.com/wikimedia/wikimedia-fundraising-SmashPig/blob/master/PaymentData/ReferenceData/CurrencyRates.php
// Updated 2023-01-27
frb.amounts.minimums = {
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : 1,
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : 1.35,
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : 1.45,
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : 1.56,
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : 0.81,
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : 0.92,
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : 6.88,
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : 365,
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : 3.40,
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : 10, // T309818
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : 128,
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : 4.31,
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : 9.92,
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : 4.36,
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : 22,
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : 4.55,
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : 37,
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : 17,
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : 5.19,
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : 183,
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : 825,
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : 4684,
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : 19,
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : 3.80,
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : 39,
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : 0.92
};

frb.amounts.options7 = {
    // Big English
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : [2.75, 10, 15, 25, 50, 75, 100],
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : [2.75, 10, 15, 25, 50, 75, 100],
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : [2.75, 10, 15, 25, 50, 75, 100],
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : [2.75, 10, 15, 25, 50, 75, 100],
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : [2, 10, 15, 25, 50, 75, 100],
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : [2, 10, 15, 25, 50, 75, 100]
    },
    // Others
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : [20, 100, 150, 200, 300, 500, 750],
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : [500, 1000, 2000, 4000, 5000, 7000, 10000],
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : [10, 35, 50, 100, 200, 300, 400],
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : [25, 300, 500, 1000, 1500, 3000, 5000],
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : [300, 1000, 1500, 2000, 3000, 5000, 10000],
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : [10, 30, 50, 100, 200, 300, 500],
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : [20, 100, 150, 200, 500, 750, 1000],
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : [10, 20, 50, 100, 200, 300, 500],
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : [50, 100, 250, 500, 1000, 1500, 2500],
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : [10, 50, 75, 100, 200, 300, 500],
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : [30, 50, 100, 200, 300, 500, 1000],
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : [50, 75, 150, 300, 500, 750, 1000],
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : [30, 50, 100, 200, 300, 500, 1000],
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : [10, 20, 30, 50, 100, 200, 300],
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : [200, 250, 500, 750, 1000, 1500, 2000],
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : [2000, 3000, 5000, 10000, 20000, 30000, 50000],
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : [10000, 15000, 25000, 50000, 100000, 150000, 200000],
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : [40, 70, 150, 250, 500, 700, 1000],
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : [10, 15, 25, 50, 100, 150, 200],
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : [100, 200, 300, 500, 1000, 1500, 2000],
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : [3, 5, 10, 25, 50, 100, 200]
};

// 5 amount options. Since 2020 6C, no longer used
frb.amounts.options5 = {
    // Big English
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : [2.75, 15, 20, 50, 100],
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : [2.75, 15, 20, 50, 100],
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : [2.75, 15, 20, 50, 100],
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : [2.75, 15, 20, 50, 100],
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : [2, 10, 20, 50, 100],
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : [2, 10, 20, 50, 100],
    // Others
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : [20, 100, 200, 500, 1000],
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : [500, 2500, 4000, 7000, 10000],
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : [10, 50, 200, 600, 1000],
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : [150, 500, 1000, 3000, 5000],
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : [300, 1500, 2000, 5000, 10000],
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : [10, 50, 100, 300, 500],
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : [20, 100, 200, 500, 1000],
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : [10, 50, 100, 300, 500],
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : [10, 50, 100, 200, 1000],
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : [30, 100, 200, 500, 1000],
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : [50, 150, 300, 750, 1000],
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : [30, 100, 200, 500, 1000],
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : [10, 30, 50, 100, 250],
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : [60, 200, 400, 1000, 2000],
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : [1500, 5000, 10000, 25000, 50000],
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : [7000, 20000, 50000, 150000, 200000],
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : [35, 100, 200, 750, 1000],
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : [10, 50, 150, 300, 700],
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : [70, 200, 400, 1500, 2000],
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : [3, 10, 25, 50, 100]
};

// &quot;Average&quot; donation
frb.amounts.averages = {
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : 13,
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : 12,
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : 11,
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : 12,
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : 6,
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : 8,
    // Others
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : 60,
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : 2500,
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : 229,
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : 800,
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : 75,
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : 150,
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : 85,
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : 150,
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : 65,
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : 25,
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : 780,
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : 10200,
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : 35000,
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : 140,
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : 30,
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : 525,
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : 25
};

// &quot; , &quot;'&quot; , &quot;If everyone gave X&quot; , &quot;'&quot; , &quot;. Mostly the same as first asks option.
frb.amounts.ifEveryone = {
    // Big English
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : 2.75,
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : 2.75,
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : 2.75,
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : 2.75,
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : 2,
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : 2
    },
    // Others
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : 20,
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : 500,
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : 25,
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : 300,
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : 20,
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : 30,
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : 50,
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : 30,
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : 175,
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : 1500,
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : 7000,
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : 40,
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : 100,
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : 5
};

// Minimum fee/PTF amounts. Default is 0.35.
// Updated 2018-07-05 based on Ppena&quot; , &quot;'&quot; , &quot;s feedback
// Updated 2019-05-21 to approx 0.35 USD equivalent
frb.amounts.feeMinimums = {
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : 2,
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : 100,
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : 1.2,
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : 4,
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : 35,
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : 1,
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : 3,
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : 1.35,
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : 7.5,
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : 1.5,
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : 3,
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : 10,
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : 5,
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : 2,
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : 32,
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : 255,
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : 1300,
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : 7.4,
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : 1.3,
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : 15.5
};

// If one-time amount &lt;= left amount, suggest right amount for monthly
// If changing these, please update spreadsheet
// https://docs.google.com/spreadsheets/d/1z36zi8EegPLAvR5FYAgwz8ywKZ50QNB82SpwpTdk-xQ/edit#gid=1258723967
frb.amounts.monthlySuggest = {
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : [ // also GBP
        [ 1.99, 0 ],
        [ 2.35, 1.40 ],
        [ 9, 1.75 ],
        [ 12, 2 ],
        [ 15, 2.5 ],
        [ 18, 3 ],
        [ 21, 3.5 ],
        [ 24, 4 ],
        [ 27, 4.5 ],
        [ 30, 5 ],
        [ 33, 5.5 ],
        [ 36, 6 ],
        [ 39, 6.5 ],
        [ 42, 7 ],
        [ 45, 7.5 ],
        [ 48, 8 ],
        [ 51, 8.5 ],
        [ 54, 9 ],
        [ 57, 9.5 ],
        [ 60, 10 ],
        [ 63, 10.5 ],
        [ 66, 11 ],
        [ 69, 11.5 ],
        [ 72, 12 ],
        [ 75, 12.5 ],
        [ 102, 17 ],
        [ 250, 25 ],
        [ 499, 50 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : [ // also CAD, AUD, NZD
        [ 2.74, 0 ],
        [ 9, 1.75 ],
        [ 12, 2 ],
        [ 15, 2.5 ],
        [ 18, 3 ],
        [ 21, 3.5 ],
        [ 24, 4 ],
        [ 27, 4.5 ],
        [ 30, 5 ],
        [ 33, 5.5 ],
        [ 36, 6 ],
        [ 39, 6.5 ],
        [ 42, 7 ],
        [ 45, 7.5 ],
        [ 48, 8 ],
        [ 51, 8.5 ],
        [ 54, 9 ],
        [ 57, 9.5 ],
        [ 60, 10 ],
        [ 63, 10.5 ],
        [ 66, 11 ],
        [ 69, 11.5 ],
        [ 72, 12 ],
        [ 75, 12.5 ],
        [ 102, 17 ],
        [ 250, 25 ],
        [ 499, 50 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : [
        [ 299, 0 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2100, 350 ],
        [ 2400, 400 ],
        [ 2700, 450 ],
        [ 3000, 500 ],
        [ 3300, 550 ],
        [ 3600, 600 ],
        [ 3900, 650 ],
        [ 4200, 700 ],
        [ 4500, 750 ],
        [ 4800, 800 ],
        [ 5100, 850 ],
        [ 5400, 900 ],
        [ 5700, 950 ],
        [ 6000, 1000 ],
        [ 6300, 1050 ],
        [ 6600, 1100 ],
        [ 6900, 1150 ],
        [ 7200, 1200 ],
        [ 7500, 1250 ],
        [ 10800, 1800 ],
        [ 18000, 3000 ],
        [ 50000, 6000 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : [
        [ 25, 0 ],
        [ 50, 25 ],
        [ 100, 30 ],
        [ 200, 50 ],
        [ 300, 70 ],
        [ 500, 90 ],
        [ 1000, 110 ],
        [ 2500, 250 ],
        [ 5000, 500 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : [
        [ 499, 0 ],
        [ 3000, 500 ],
        [ 6000, 1000 ],
        [ 9000, 1500 ],
        [ 12000, 2000 ],
        [ 18000, 3000 ],
        [ 24000, 4000 ],
        [ 30000, 5000 ],
        [ 36000, 6000 ],
        [ 42000, 7000 ],
        [ 48000, 8000 ],
        [ 54000, 9000 ],
        [ 60000, 10000 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : [
        [ 9, 0 ],
        [ 10, 5 ],
        [ 60, 10 ],
        [ 90, 15 ],
        [ 120, 20 ],
        [ 180, 30 ],
        [ 240, 40 ],
        [ 300, 50 ],
        [ 360, 60 ],
        [ 420, 70 ],
        [ 480, 80 ],
        [ 540, 90 ],
        [ 600, 100 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : [
        [ 29, 0 ],
        [ 30, 20 ],
        [ 50, 30 ],
        [ 100, 40 ],
        [ 300, 50 ],
        [ 450, 75 ],
        [ 600, 100 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2400, 400 ],
        [ 3000, 500 ],
        [ 3600, 600 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : [ // Also RON, PLN
        [ 9, 0 ],
        [ 30, 5 ],
        [ 50, 10 ],
        [ 120, 20 ],
        [ 180, 30 ],
        [ 240, 40 ],
        [ 300, 50 ],
        [ 360, 60 ],
        [ 420, 70 ],
        [ 480, 80 ],
        [ 540, 90 ],
        [ 600, 100 ],
        [ 900, 150 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : [ // Also NOK
        [ 19, 0 ],
        [ 20, 10 ],
        [ 120, 20 ],
        [ 180, 30 ],
        [ 300, 50 ],
        [ 450, 75 ],
        [ 600, 100 ],
        [ 750, 125 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2100, 350 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : [
        [ 49, 0 ],
        [ 180, 30 ],
        [ 300, 50 ],
        [ 600, 100 ],
        [ 900, 150 ],
        [ 1200, 200 ],
        [ 1500, 250 ],
        [ 1800, 300 ],
        [ 2100, 350 ],
        [ 2400, 400 ],
        [ 3000, 500 ],
        [ 3600, 600 ],
        [ 4200, 700 ],
        [ 4800, 800 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : [
        [ 1999, 0 ],
        [ 2300, 1000 ],
        [ 2700, 1100 ],
        [ 3300, 1200 ],
        [ 4200, 1300 ],
        [ 5500, 1400 ],
        [ 9000, 1500 ],
        [ 10500, 1700 ],
        [ 16000, 2600 ],
        [ 20800, 3400 ],
        [ 26000, 4200 ],
        [ 31200, 5000 ],
        [ 38400, 6400 ],
        [ 55000, 8500 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : [
        [ 99, 0 ],
        [ 120, 50 ],
        [ 170, 65 ],
        [ 220, 70 ],
        [ 320, 75 ],
        [ 480, 85 ],
        [ 520, 90 ],
        [ 750, 125 ],
        [ 1050, 170 ],
        [ 1350, 225 ],
        [ 1600, 250 ],
        [ 1800, 300 ],
        [ 2100, 320 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : [
        [ 9, 0 ],
        [ 12, 5 ],
        [ 17, 6 ],
        [ 26, 7 ],
        [ 48, 8 ],
        [ 55, 9 ],
        [ 78, 13 ],
        [ 105, 17 ],
        [ 130, 21 ],
        [ 160, 26 ],
        [ 180, 30 ],
        [ 210, 32 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : [
        [ 9999, 0 ],
        [ 11300, 5000 ],
        [ 17000, 5200 ],
        [ 22000, 5500 ],
        [ 27000, 5800 ],
        [ 45000, 7500 ],
        [ 55000, 9000 ],
        [ 75000, 12500 ],
        [ 105000, 17000 ],
        [ 120000, 20000 ],
        [ 160000, 25000 ],
        [ 180000, 30000 ],
        [ 250000, 34000 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : [
        [ 9, 0 ],
        [ 12, 6 ],
        [ 22, 7 ],
        [ 35, 8 ],
        [ 45, 9 ],
        [ 55, 10 ],
        [ 80, 12 ],
        [ 105, 16 ],
        [ 160, 25 ],
        [ 210, 35 ],
        [ 270, 45 ],
        [ 320, 50 ],
        [ Infinity, 0 ]
    ],
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : [
        [ 39, 0 ],
        [ 48, 25 ],
        [ 60, 28 ],
        [ 110, 30 ],
        [ 160, 35 ],
        [ 260, 45 ],
        [ 270, 50 ],
        [ 350, 60 ],
        [ 550, 85 ],
        [ 650, 90 ],
        [ 750, 120 ],
        [ 1500, 160 ],
        [ Infinity, 0 ]
    ]
    
};
frb.amounts.monthlySuggest.GBP = frb.amounts.monthlySuggest.EUR;
frb.amounts.monthlySuggest.CAD = frb.amounts.monthlySuggest.USD;
frb.amounts.monthlySuggest.AUD = frb.amounts.monthlySuggest.USD;
frb.amounts.monthlySuggest.NZD = frb.amounts.monthlySuggest.USD;

frb.amounts.monthlySuggest.RON = frb.amounts.monthlySuggest.MYR;
frb.amounts.monthlySuggest.PLN = frb.amounts.monthlySuggest.MYR;
frb.amounts.monthlySuggest.NOK = frb.amounts.monthlySuggest.DKK;

frb.currencyFormats = {
    &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;fr&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;£\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;cy&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ga&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;mt&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;nl&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€ \t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;lv&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€ \t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;tr&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;€ \t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t €&quot; , &quot;'&quot; , &quot;
    },
    // Others
    &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t Kč&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t kr.&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t Ft&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;he&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t ₪&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;yi&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t ₪&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ar&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t ₪&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;₪ \t&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;₹ \t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;¥\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;RM\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t kr&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t zł&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t lei&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t kr&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;₴\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;R \t&quot; , &quot;'&quot; , &quot;,
    // Latin America
    &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;R$\t&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;R$ \t&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$\t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;S/. \t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;$U \t&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;\t CHF&quot; , &quot;'&quot; , &quot;
};

// Check in user language first, then fall back to English
frb.countryNames = {
    &quot; , &quot;'&quot; , &quot;af&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Suid-Afrika&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;the United States&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Canada&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;the UK&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Ireland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Australia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;New Zealand&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Argentina&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Austria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Belgium&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Brazil&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Switzerland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Chile&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Colombia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;the Czech Republic&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Denmark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Spain&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;France&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Greece&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Hong Kong&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Hungary&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;India&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Italy&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Japan&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Luxembourg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Latvia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Mexico&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Malaysia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;the Netherlands&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Norway&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Peru&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Poland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Romania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Sweden&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Slovakia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Ukraine&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Uruguay&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;South Africa&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;ca&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;d’Àustria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Bèlgica&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Dinamarca&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;a Espanya&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;d’Hongria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;d’Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Letònia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Luxemburg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Malàisia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Noruega&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Polònia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Romania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;d’Eslovàquia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;de Sud-àfrica&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;d’Ucraïna&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;cs&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v České republice&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Rakousku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Belgii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Dánsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Řecku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Izraeli&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Lucembursku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Malajsii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Norsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Portugalsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ve Švédsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Jihoafrické republice&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;es&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Argentina&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Austria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Bélgica&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Brasil&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Chile&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Colombia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Dinamarca&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en España&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Hungría&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Luxemburgo&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Letonia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en México&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Malasia &quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Noruega&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Perú&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Polonia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Rumania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Eslovaquia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Ucrania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en los Estados Unidos&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Uruguay&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Sudafrica&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;da&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Østrig&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Belgien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Tjekkiet&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Danmark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Spanien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Grækenland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ungarn&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Letland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Luxembourg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Malaysia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Norge&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Polen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Rumænien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sverige&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Slovakiet&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sydafrika&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ukraine&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;nl&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Nederland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Oostenrijk&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in België&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Denemarken&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Tsjechië&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Spanje&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Griekenland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Hongarije&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Israël&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Letland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Luxemburg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Maleisië&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Noorwegen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Polen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Roemenië&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Zweden&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Slowakije&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Zuid-Afrika&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Oekraïne&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;fr&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Autriche&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Belgique&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Suisse&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;au Canada&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en République tchèque&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;au Danemark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Espagne&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en France&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Grèce&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Hongrie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Israël&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Lettonie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;au Luxembourg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Malaisie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Norvège&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Pologne&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;au Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Roumanie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Suède&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Slovaquie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Afrique du Sud&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;en Ukraine&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;de&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Österreich&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Belgien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in der Schweiz&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Tschechien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Dänemark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Spanien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Griechenland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Ungarn&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Lettland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Luxemburg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Malaysia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Norwegen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Polen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Rumänien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Schweden&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in der Slowakei&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Südafrika&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in der Ukraine&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;el&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Αυστρία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στο Βέλγιο&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Τσεχία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Δανία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Ισπανία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Ελλάδα&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Ουγγαρία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στο Ισραήλ&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Λετονία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στο Λουξεμβούργο&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Μαλαισία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Νορβηγία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Πολωνία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Πορτογαλία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Ρουμανία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Σουηδία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Σλοβακία&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στη Νότια Αφρική&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;στην Ουκρανία&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;he&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;אוסטרליה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;בלגיה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot;בצ&quot; , &quot;'&quot; , &quot;כיה&quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;דנמרק&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ספרד&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ביוון&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;הונגריה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ישראל&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;לטביה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;לוקסמבורג&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;מלזיה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;נורווגיה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;פולין&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;פורטוגל&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;רומניה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;בשוודיה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;סלובקיה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;דרום אפריקה&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;אוקראינה&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;hu&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ausztriai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;belgiumi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;dániai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;spanyolországi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;magyarországi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;izraeli&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;lettországi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;luxemburgi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;malajziai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;norvégiai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;lengyelországi&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;portugáliai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;romániai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;szlovákiai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;dél-afrikai&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;ukrajnai&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;it&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Italia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Svizzera&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;lv&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Austrijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Beļģijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Dānijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Spānijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Ungārijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Izraēlas valstī&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Latvijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Luksemburgā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Malaizijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Norvēģijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Polijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Portugālē&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Rumānijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Slovākijā&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;Dienvidāfrikas valstī&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;valstī Ukrainā&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;nb&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Østerrike&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Belgia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Tsjekkia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Danmark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Spania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Hellas&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ungarn&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Latvia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Luxembourg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Malaysia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Norge&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Polen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Romania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sverige&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Slovakia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sør-Afrika&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ukraina&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;pl&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Austrii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Belgii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Danii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Hiszpanii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Węgrzech&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Izraelu&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Łotwie&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Luksemburgu&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Malezji&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Norwegii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Polsce&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Portugalii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Rumunii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Słowacji&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;w Republice Południowej Afryki&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Ukrainie&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Áustria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Bélgica&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;no Brasil&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na República Checa&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Dinamarca&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Espanha&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Grécia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Hungria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;em Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Letónia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;no Luxemburgo&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Malásia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Noruega&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Polónia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;em Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Roménia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Suécia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Eslováquia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na África do Sul&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Ucrânia&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Austria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Belgia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Danemarca&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;în Spania&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Ungaria&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Latvia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Luxemburg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Malaezia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Norvegia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Polonia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Portugalia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din România&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Slovacia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Africa de Sud&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;din Ucraina&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Австрии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Бельгии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Дании&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Испании&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Венгрии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Израиле&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Латвии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Люксембурге&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Малайзии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Норвегии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Польше&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Португалии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Румынии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Словакии&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Южной Африке&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Украине&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;sk&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Rakúsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Belgicku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Dánsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Španielsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Maďarsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Izraeli&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Lotyšsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Luxembursku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Malajzii&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Nórsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Poľsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Portugalsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Rumunsku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Slovensku&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;v Juhoafrickej republike&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;na Ukrajine&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;sv&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sverige&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Österrike&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Belgien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Tjeckien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Danmark&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Spanien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Grekland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ungern&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Israel&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Lettland&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Luxemburg&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Malaysia&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Norge&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Polen&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Portugal&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Rumänien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Slovakien&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Sydafrika&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;i Ukraina&quot; , &quot;'&quot; , &quot;
    },
    &quot; , &quot;'&quot; , &quot;uk&quot; , &quot;'&quot; , &quot; : {
        &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Австрії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Бельгії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Данії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Іспанії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Угорщині&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Ізраїлі&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Латвії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Люксембургу&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Малайзії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Норвегії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Польщі&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Португалії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Румунії&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у Словаччині&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;у ПАР&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;в Україні&quot; , &quot;'&quot; , &quot;
    }
};

/* 
Most of the translations are actually using &quot;in COUNTRY&quot; or similar to account for grammar differences.
So this makes English do the same, and allows us to use a clearer %in-country% variable, while avoiding
breaking old content using %country%.
*/
frb.inCountryNames = JSON.parse( JSON.stringify( frb.countryNames ) ); // deep copy
frb.inCountryNames.en = {
    &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in the United States&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Canada&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in the UK&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Ireland&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Australia&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in New Zealand&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Argentina&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Austria&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Belgium&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Brazil&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Switzerland&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Chile&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Colombia&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in the Czech Republic&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Denmark&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Spain&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in France&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Greece&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;HK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Hong Kong&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Hungary&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Israel&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in India&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Italy&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Japan&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Luxembourg&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Latvia&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Mexico&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Malaysia&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in the Netherlands&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Norway&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Peru&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Poland&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Portugal&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Romania&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Sweden&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Slovakia&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Ukraine&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in Uruguay&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;in South Africa&quot; , &quot;'&quot; , &quot;
};

frb.dayNames = {
    &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;Sunday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Monday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Tuesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Wednesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Thursday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Friday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Saturday&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;ca&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;diumenge&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dilluns&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dimarts&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dimecres&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dijous&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;divendres&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dissabte&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;ja&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;月&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;火&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;水&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;木&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;金&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;土&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;es&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;domingo&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lunes&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;martes&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;miércoles&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;jueves&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;viernes&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sábado&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;sv&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;söndag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;måndag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tisdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;onsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;torsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;fredag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lördag&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;da&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;søndag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mandag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tirsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;onsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;torsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;fredag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lørdag&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;nb&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;søndagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mandagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tirsdagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;onsdagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;torsdagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;fredagen&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lørdagen&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;it&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;domenica&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lunedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;martedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mercoledì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;giovedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;venerdì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sabato&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;nl&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;zondag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;maandag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;dinsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;woensdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;donderdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;vrijdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;zaterdag&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;fr&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;dimanche&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;lundi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mardi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mercredi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;jeudi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;vendredi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;samedi&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;de&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;Sonntag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Montag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Dienstag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Mittwoch&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Donnerstag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Freitag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Samstag&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;he&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;ראשון&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;שני&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;שלישי&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;רביעי&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;חמישי&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;שישי&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;שבת&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;lv&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;svētdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;pirmdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;otrdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;trešdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ceturtdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;piektdienā&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sestdienā&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;pl&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;niedzielę&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;poniedziałek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;wtorek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;środę&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;czwartek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;piątek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sobotę&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;neste domingo&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nesta segunda-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nesta terça-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nesta quarta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nesta  quinta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nesta sexta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;neste sábado&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;воскресенье&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;понедельник&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;вторник&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;среду&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;четверг&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;пятницу&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;субботу&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;uk&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;неділі&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;понеділка&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;вівторка&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;середи&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;четверга&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;п’ятниц&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;суботи&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;hu&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;vasárnap&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;hétfő&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;kedd&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;szerda&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;csütörtök&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;péntek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;szombat&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;ro&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;duminică&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;luni&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;marți&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;miercuri&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;joi&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;vineri&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sâmbătă&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;af&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;Sondag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Maandag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Dinsdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Woensdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Donderdag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Vrydag&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Saterdag&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;aa&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;Sunday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Monday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Tuesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Wednesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Thursday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Friday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Saturday&quot; , &quot;'&quot; , &quot; ]
};

// &quot;This fooday&quot; translations. Needed for some languages where gender varies and &quot;this&quot; must agree
frb.dayNamesThis = {
    &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;this Sunday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Monday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Tuesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Wednesday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Thursday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Friday&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;this Saturday&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;el&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;Αυτήν την	Κυριακή&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτή τη Δευτέρα&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτήν την	Τρίτη&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτήν την Τετάρτη&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτήν την	Πέμπτη&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτήν την Παρασκευή&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;Αυτό το Σάββατο&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;jp&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;この日曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この月曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この火曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この水曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この木曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この金曜日&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;この土曜日&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;it&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;questa domenica&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo lunedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo martedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo mercoledì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo giovedì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo venerdì&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;questo sabato&quot; , &quot;'&quot; , &quot;],
    &quot; , &quot;'&quot; , &quot;pl&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;w tę niedzielę&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w ten poniedziałek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w ten wtorek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w tę środę&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w ten czwartek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w ten piątek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;w tę sobotę&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;ru&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;в это воскресенье&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в этот понедельник&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в этот вторник&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в эту среду&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в этот четверг&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в эту пятницу&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;в эту субботу&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;uk&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;цієї неділі&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цього понеділка&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цього вівторка&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цієї середи&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цього четверга&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цієї п’ятниці&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;цієї суботи&quot; , &quot;'&quot; , &quot; ],
    &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;este domingo&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;esta segunda-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;esta terça-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;esta quarta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;esta quinta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;esta sexta-feira&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;este sábado&quot; , &quot;'&quot; , &quot;],
    &quot; , &quot;'&quot; , &quot;sk&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;túto nedeľu&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento pondelok&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento utorok&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;túto stredu&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento štvrtok&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento piatok&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;túto sobotu&quot; , &quot;'&quot; , &quot;],
    &quot; , &quot;'&quot; , &quot;cs&quot; , &quot;'&quot; , &quot; : [ &quot; , &quot;'&quot; , &quot;tuto neděli&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;toto pondělí&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;toto úterý&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tuto středu&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento čtvrtek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tento pátek&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;tuto sobotu&quot; , &quot;'&quot; , &quot;]
};

frb.iPadTranslations = {
    &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;iPad&quot; , &quot;'&quot; , &quot;
};

// Insert any localize data overrides here

/* jshint maxerr: 600 */
/* MediaWiki:FundraisingBanners/CoreJS-2018.js
 * Core code for banner forms, with new inline error messages
 */

var frb = frb || {};

/**
 * Test for general ES6 support
 *
 * Checks for arrow functions, default parameters, NodeList.prototype.forEach()
 * Should be roughly Chrome 51+, Firefox 50+, Edge 16+, Safari 10+
 * Based on https://gist.github.com/bendc/d7f3dbc83d0f65ca0433caf90378cd95
 * @return {boolean}
 */
frb.supportedBrowser = function() {
    try {
        new Function(&quot; , &quot;'&quot; , &quot;(a = 0) => a&quot; , &quot;'&quot; , &quot;);
        document.querySelectorAll(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;).forEach(a => a);
        return true;
    }
    catch (err) {
        return false;
    }
}();

if ( !mw.centralNotice.adminUi ) { // T262693
    frb.loadedTime = Date.now();
    frb.didSelectAmount = false;
    frb.optinRequiredCountries =
        [ &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot;,
          &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot; ];
    frb.optinRequired = frb.optinRequiredCountries.indexOf(mw.centralNotice.data.country) !== -1;
    frb.maxUSD = 25000;
    frb.reduceMotion = window.matchMedia(&quot; , &quot;'&quot; , &quot;(prefers-reduced-motion: reduce)&quot; , &quot;'&quot; , &quot;).matches;
}

// Keyboard shortcut to go from banner preview to editor - Ctrl+Shift+E
if ( mw.config.get(&quot; , &quot;'&quot; , &quot;wgUserName&quot; , &quot;'&quot; , &quot;) ) {
    if ( mw.config.get(&quot; , &quot;'&quot; , &quot;wgUserName&quot; , &quot;'&quot; , &quot;).match(/\(WMF\)/) ) {
        window.addEventListener(&quot; , &quot;'&quot; , &quot;keydown&quot; , &quot;'&quot; , &quot;, function(e) {
            if ( e.ctrlKey &amp;&amp; e.shiftKey &amp;&amp; e.keyCode === 69 ) {
                window.open( &quot; , &quot;'&quot; , &quot;https://meta.wikimedia.org/wiki/Special:CentralNoticeBanners/Edit/&quot; , &quot;'&quot; , &quot; + mw.centralNotice.data.banner );
            }
        });
    }
}

/**
 * Main function to submit to paymentswiki
 *
 * @param  {Object} options
 * - method (required)
 * - submethod (optional)
 * - gateway (optional)
 * - skipValidation (optional boolean, for pp-usd. Not yet implemented.)
 * @param  {Boolean} isEndowment - deprecated, set frb.isEndowment instead
 */
frb.submitForm = function( options, isEndowment ) {

    var uri = new mw.Uri(&quot; , &quot;'&quot; , &quot;https://payments.wikimedia.org/index.php/Special:GatewayChooser&quot; , &quot;'&quot; , &quot;);
    var params = {};

    if ( !frb.validateForm( options ) ) {
        frb.extraData.validateError = 1; // Flag they had an error, even if fixed later
        return false; // Error, bail out of submitting
    }

    // Skip form chooser for Apple Pay / Google Pay
    if ( options.method === &quot; , &quot;'&quot; , &quot;apple&quot; , &quot;'&quot; , &quot; || options.method === &quot; , &quot;'&quot; , &quot;google&quot; , &quot;'&quot; , &quot; ) {
        uri = new mw.Uri(&quot; , &quot;'&quot; , &quot;https://payments.wikimedia.org/index.php/Special:AdyenCheckoutGateway&quot; , &quot;'&quot; , &quot;);
    }

    // Skip form chooser for Venmo
    if ( options.method === &quot; , &quot;'&quot; , &quot;venmo&quot; , &quot;'&quot; , &quot; ) {
        uri = new mw.Uri(&quot; , &quot;'&quot; , &quot;https://payments.wikimedia.org/index.php/Special:BraintreeGateway&quot; , &quot;'&quot; , &quot;);
    }

    // Form selection data
    params.payment_method = options.method;
    if ( options.submethod ) {
        params.payment_submethod = options.submethod;
    }
    if ( options.gateway ) {
        params.gateway = options.gateway;
    }
    if ( options.variant ) {
        params.variant = options.variant;
    }
    params.recurring = frb.getRecurring();

    if ( params.recurring &amp;&amp; params.variant &amp;&amp; params.variant.match( /monthlyConvert/ ) ) {
        // Post-payments monthly convert makes no sense if it&quot; , &quot;'&quot; , &quot;s already recurring
        // Avoid things like T312905
        delete params.variant;
    }

    params.currency = frb.getCurrency(mw.centralNotice.data.country) || &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot;;

    params.uselang = mw.centralNotice.data.uselang || &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;;
    params.country = mw.centralNotice.data.country || &quot; , &quot;'&quot; , &quot;XX&quot; , &quot;'&quot; , &quot;;

    if ( params.uselang === &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot; &amp;&amp; params.country === &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; ) {
        params.uselang = &quot; , &quot;'&quot; , &quot;pt-br&quot; , &quot;'&quot; , &quot;;
    }
    if ( params.uselang === &quot; , &quot;'&quot; , &quot;es&quot; , &quot;'&quot; , &quot; &amp;&amp;
        ( params.country === &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot; || params.country === &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot; ||
          params.country === &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot; || params.country === &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot; ||
          params.country === &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot; || params.country === &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot; ||
          params.country === &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; )
    ) {
        params.uselang = &quot; , &quot;'&quot; , &quot;es-419&quot; , &quot;'&quot; , &quot;;
    }

    // Adyen override. frb.ccAdyenCountries is defined in LocalizeJS-2017.js
    if ( params.payment_method === &quot; , &quot;'&quot; , &quot;cc&quot; , &quot;'&quot; , &quot; &amp;&amp; frb.ccAdyenCountries.indexOf( params.country ) !== -1 ) {
        params.gateway = &quot; , &quot;'&quot; , &quot;adyen&quot; , &quot;'&quot; , &quot;;
    }
    // dLocal override for South Africa
    if ( params.payment_method === &quot; , &quot;'&quot; , &quot;cc&quot; , &quot;'&quot; , &quot; &amp;&amp; params.country === &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; ) {
        params.gateway = &quot; , &quot;'&quot; , &quot;astropay&quot; , &quot;'&quot; , &quot;;
    }

    // Amount
    var amount = frb.getAmount();
    if ( $(&quot; , &quot;'&quot; , &quot;#frb-ptf-checkbox&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;) ) {
        amount = amount + frb.calculateFee(amount);
        frb.extraData.ptf = 1;
    }
    params.amount = amount;

    // Email optin
    if ( frb.optinRequired &amp;&amp; $(&quot; , &quot;'&quot; , &quot;input[name=&quot;opt_in&quot;]&quot; , &quot;'&quot; , &quot;).length > 0 ) {
        var opt_inValue = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;opt_in&quot;]:checked&quot; , &quot;'&quot; , &quot;).val();
        params.opt_in   = opt_inValue; // frb.validateForm() already checked it&quot; , &quot;'&quot; , &quot;s 1 or 0
    }

    // Tracking info
    if ( isEndowment || frb.isEndowment ) {
        params.utm_medium = &quot; , &quot;'&quot; , &quot;endowment&quot; , &quot;'&quot; , &quot;;
        params.appeal = &quot; , &quot;'&quot; , &quot;EndowmentQuote&quot; , &quot;'&quot; , &quot;;
    } else {
        params.utm_medium = &quot; , &quot;'&quot; , &quot;sitenotice&quot; , &quot;'&quot; , &quot;;
    }
    params.utm_campaign = mw.centralNotice.data.campaign || &quot; , &quot;'&quot; , &quot;test&quot; , &quot;'&quot; , &quot;;
    params.utm_source   = frb.buildUtmSource(params);

    frb.extraData.vw = window.innerWidth;
    frb.extraData.vh = window.innerHeight;
    frb.extraData.time = Math.round( (Date.now() - frb.loadedTime)/1000 );

    if ( navigator.brave !== undefined ) { // T283367
        frb.extraData.brave = &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;;
    }

    if ( !$.isEmptyObject( frb.extraData ) ) {
        params.utm_key = frb.buildUtmKey( frb.extraData );
    }

    // Link to Banner History if enabled
    var mixins = mw.centralNotice.getDataProperty( &quot; , &quot;'&quot; , &quot;mixins&quot; , &quot;'&quot; , &quot; );
    if ( mixins &amp;&amp; mixins.bannerHistoryLogger ) {
        params.bannerhistlog = mw.centralNotice.bannerHistoryLogger.id;
    }

    uri.extend(params);

    // Set a cookie with current location so we can return here from TY page
    mw.loader.using( [ &quot; , &quot;'&quot; , &quot;mediawiki.cookie&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;mediawiki.util&quot; , &quot;'&quot; , &quot; ] ).then( function () {
        // Exclude URL parameters like banner, but cope with paths like /w/index.php?title=Foo
        var returnToUrl = window.location.origin + mw.util.getUrl();
        mw.cookie.set(
            &quot; , &quot;'&quot; , &quot;fundraising_returnTo&quot; , &quot;'&quot; , &quot;,
            returnToUrl,
            { expires: 300, prefix: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, domain: &quot; , &quot;'&quot; , &quot;.wikipedia.org&quot; , &quot;'&quot; , &quot;, secure: true }
        );
    });

    if ( mixins &amp;&amp; mixins.bannerHistoryLogger ) {
        mw.centralNotice.bannerHistoryLogger.ensureLogSent().always(function() {
            frb.goToPayments( uri );
        });
    } else {
        frb.goToPayments( uri );
    }

};

frb.goToPayments = function( uri ) {
    if ( window.top !== window.self ) {
        // banner is in a frame, open payments in a new tab
        window.open( uri.toString() );
    } else {
        window.location.href = uri.toString();
    }
};

/**
 * Check the form for errors.
 *
 * Called on submission, can also be called on input
 *
 * @param {object} options
 * @return {boolean} Whether form is error-free
 */
frb.validateForm = function( options ) {
    var error = false;

    /* Reset all errors */
    $(&quot; , &quot;'&quot; , &quot;.frb-haserror&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
    $(&quot; , &quot;'&quot; , &quot;.frb-error&quot; , &quot;'&quot; , &quot;).hide();

    if ( !options.method ) {
        error = true;
        $(&quot; , &quot;'&quot; , &quot;.frb-methods&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-method&quot; , &quot;'&quot; , &quot;).show();
    }

    if ( !frb.validateAmount() ) {
        error = true;
    }

    /* Email optin */
    if ( frb.optinRequired &amp;&amp; $(&quot; , &quot;'&quot; , &quot;.frb-optin&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:visible&quot; , &quot;'&quot; , &quot;) ) {
        var opt_inValue = $(&quot; , &quot;'&quot; , &quot;input[name=&quot;opt_in&quot;]:checked&quot; , &quot;'&quot; , &quot;).val();
        if ( opt_inValue !== &quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot; &amp;&amp; opt_inValue !== &quot; , &quot;'&quot; , &quot;0&quot; , &quot;'&quot; , &quot; ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-optin&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-error-optin&quot; , &quot;'&quot; , &quot;).show();
            error = true;
        }
    }

    return !error;
};

/**
 * Check if selected amount is valid i.e. a positive number, between minimum and maximum.
 * If not, show an error and return false.
 */
frb.validateAmount = function() {

    var amount = frb.getAmount(),
        currency  = frb.getCurrency( mw.centralNotice.data.country ),
        minAmount = frb.amounts.minimums[ currency ],
        maxAmount = Math.round( frb.maxUSD * minAmount );
        // Math.round to account for floating point math errors: https://phabricator.wikimedia.org/T246262

    if ( amount === null || isNaN(amount) || amount &lt;= 0 || amount &lt; minAmount ) {
        $(&quot; , &quot;'&quot; , &quot;fieldset.frb-amounts&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-bigamount&quot; , &quot;'&quot; , &quot;).hide();
        $(&quot; , &quot;'&quot; , &quot;.frb-error-smallamount&quot; , &quot;'&quot; , &quot;).show();
        return false;
    } else if ( amount > Math.round( maxAmount ) ) {
        $(&quot; , &quot;'&quot; , &quot;fieldset.frb-amounts&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-bigamount&quot; , &quot;'&quot; , &quot;).show();
        return false;
    } else {
        $(&quot; , &quot;'&quot; , &quot;fieldset.frb-amounts&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-smallamount, .frb-error-bigamount&quot; , &quot;'&quot; , &quot;).hide();
        return true;
    }
};

/**
 * Build the utm_source for analytics.
 *
 * Own function so it can be overriden for weird tests
 *
 * @param  {Object} params
 * @return {string} utm_source
 */
frb.buildUtmSource = function(params) {

    var utm_source;
    var fullDottedPaymentMethod = params.payment_method;
    if ( params.recurring ) {
        fullDottedPaymentMethod = &quot; , &quot;'&quot; , &quot;r&quot; , &quot;'&quot; , &quot; + fullDottedPaymentMethod;
    }
    if ( params.payment_submethod ) {
        fullDottedPaymentMethod = fullDottedPaymentMethod + &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot; + params.payment_submethod;
    }

    utm_source = mw.centralNotice.data.banner;

    // Keeping opt-in in utm_source for safety for now
    // Eventually remove it, or move to utm_key?
    if ( params.opt_in ) {
        utm_source += &quot; , &quot;'&quot; , &quot;_optIn&quot; , &quot;'&quot; , &quot; + params.opt_in;
    }

    utm_source += &quot; , &quot;'&quot; , &quot;.no-LP.&quot; , &quot;'&quot; , &quot; + fullDottedPaymentMethod;

    return utm_source;
};

/**
 * Build a string for utm_key from extra tracking data
 *
 * @param  {Object} data
 * @return {string} utm_key
 */
frb.buildUtmKey = function(data) {
    var dataArray = [];
    for (var key in data) {
        if (data.hasOwnProperty(key)) {
            dataArray.push( key + &quot; , &quot;'&quot; , &quot;_&quot; , &quot;'&quot; , &quot; + data[key] );
        }
    }
    return dataArray.join(&quot; , &quot;'&quot; , &quot;~&quot; , &quot;'&quot; , &quot;);
};

/**
 * Determine if we should show recurring choice on step 2
 * 
 * NOTE 2023-12-07: we don&quot; , &quot;'&quot; , &quot;t currently use this for step 2, since there are no
 *	banners where users select method before frequency. However it is used by
 *	frb.shouldShowMonthlyConvert()
 *
 * @param  {Object} options     Including method and optional gateway
 * @param  {String} country
 * @return {boolean}
 */
frb.shouldShowRecurring = function( options, country ) {

    if ( frb.isEndowment ) {
        return false;
    }
    if ( frb.noRecurringCountries.indexOf( country ) !== -1 ) { // Defined in LocalizeJS-2017.js
        return false;
    }
    if ( options.method === undefined ) {
        return true; // Show if a method hasn&quot; , &quot;'&quot; , &quot;t been selected yet
    }
    if ( [ &quot; , &quot;'&quot; , &quot;cc&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;paypal&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;venmo&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;apple&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;google&quot; , &quot;'&quot; , &quot; ].indexOf( options.method ) !== -1 ) {
        return true;
    }
    // Adyen iDEAL
    if ( options.submethod === &quot; , &quot;'&quot; , &quot;rtbt_ideal&quot; , &quot;'&quot; , &quot; ) {
        return true;
    }
    if ( options.submethod === &quot; , &quot;'&quot; , &quot;upi&quot; , &quot;'&quot; , &quot; || options.submethod === &quot; , &quot;'&quot; , &quot;paytmwallet&quot; , &quot;'&quot; , &quot; ) {
        return true;
    }
    return false;
};

/* Is recurring method selected? This function can be overriden for different forms */
frb.getRecurring = function() {
    // Can&quot; , &quot;'&quot; , &quot;t use simple form.frequency.value, doesn&quot; , &quot;'&quot; , &quot;t work in IE
    var selected = $(&quot; , &quot;'&quot; , &quot;#frb-form input[name=&quot;frequency&quot;]:checked&quot; , &quot;'&quot; , &quot;).val();
    return selected === &quot; , &quot;'&quot; , &quot;monthly&quot; , &quot;'&quot; , &quot;;
};

/* Return amount selected */
frb.getAmount = function() {
    var form = document.getElementById(&quot; , &quot;'&quot; , &quot;frb-form&quot; , &quot;'&quot; , &quot;);
    var amount = null;
    frb.extraData.otherAmt = 0;

    // If there are some amount radio buttons, then look for the checked one
    if (form.amount) {
        for (var i = 0; i &lt; form.amount.length; i++) {
            if (form.amount[i].checked) {
                amount = form.amount[i].value;
            }
        }
    }

    // Check the &quot;other&quot; amount box
    if (form.otherAmount.value !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
        var otherAmount = form.otherAmount.value;
        otherAmount = otherAmount.replace(/[,.](\d)$/, &quot; , &quot;'&quot; , &quot;:$10&quot; , &quot;'&quot; , &quot;);
        otherAmount = otherAmount.replace(/[,.](\d)(\d)$/, &quot; , &quot;'&quot; , &quot;:$1$2&quot; , &quot;'&quot; , &quot;);
        otherAmount = otherAmount.replace(/[$£€¥,.]/g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        otherAmount = otherAmount.replace(/:/, &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;);
        amount = otherAmount;
        frb.extraData.otherAmt = 1;
    }

    amount = parseFloat(amount);

    if ( isNaN(amount) ) {
        return 0;
    } else {
        return amount;
    }

};

/* Localize the amount errors. Call when initialising banner. */
frb.localizeErrors = function() {
    var currency  = frb.getCurrency( mw.centralNotice.data.country ),
        language = mw.centralNotice.data.uselang,
        minAmount = frb.amounts.minimums[ currency ],
        maxAmount = Math.round( frb.maxUSD * minAmount );
        // Math.round to account for floating point math errors: https://phabricator.wikimedia.org/T246262

    $(&quot; , &quot;'&quot; , &quot;.frb-error-smallamount&quot; , &quot;'&quot; , &quot;).text( function( index, oldText ) {
        return oldText.replace( &quot; , &quot;'&quot; , &quot;$1&quot; , &quot;'&quot; , &quot;, frb.formatCurrency(currency, minAmount, language)  );
    });

    $(&quot; , &quot;'&quot; , &quot;.frb-error-bigamount&quot; , &quot;'&quot; , &quot;).text( function( index, oldText ) {
        // We cannot accept donations greater than $1 $2 through our website. Please contact our major gifts staff at $3.
        return oldText.replace( &quot; , &quot;'&quot; , &quot;$1&quot; , &quot;'&quot; , &quot;, maxAmount )
                      .replace( &quot; , &quot;'&quot; , &quot;$2&quot; , &quot;'&quot; , &quot;, currency )
                      .replace( &quot; , &quot;'&quot; , &quot;$3&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;benefactors@wikimedia.org&quot; , &quot;'&quot; , &quot; );
    });
};

/**
 * Shared code for amount input handling
 */
frb.initAmountOptions = function() {

    // Reset &quot;Other&quot; input if user clicks a preset amount
    $(&quot; , &quot;'&quot; , &quot;#frb-form [id^=frb-amt-ps]&quot; , &quot;'&quot; , &quot;).click(function() {
        $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
    });

    // Track if they selected and then later changed amount
    var checkAmountChange = function(e) {
        if ( frb.didSelectAmount ) {
            frb.extraData.changedAmt = 1;
        }
        // check if amount radio button is selected OR there is a value in the other amount
        if ( $(&quot; , &quot;'&quot; , &quot;.frb-amounts input[type=&quot;radio&quot;]:checked&quot; , &quot;'&quot; , &quot;).val() !== &quot; , &quot;'&quot; , &quot;Other&quot; , &quot;'&quot; , &quot; || $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).val().length > 0 ) {
            frb.didSelectAmount = true;
        }
        return;
    };

    $(&quot; , &quot;'&quot; , &quot;.frb-amounts input[type=&quot;radio&quot;]&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, checkAmountChange);
    $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;focusout&quot; , &quot;'&quot; , &quot;, checkAmountChange);

    // Block typing non-numerics in input field, otherwise Safari allows them and then chokes
    // https://phabricator.wikimedia.org/T118741, https://phabricator.wikimedia.org/T173431
    var blockNonNumeric = function(e) {
        // Allow special keys in Firefox
        if ((e.code == &quot; , &quot;'&quot; , &quot;ArrowLeft&quot; , &quot;'&quot; , &quot;) || (e.code == &quot; , &quot;'&quot; , &quot;ArrowRight&quot; , &quot;'&quot; , &quot;) ||
            (e.code == &quot; , &quot;'&quot; , &quot;ArrowUp&quot; , &quot;'&quot; , &quot;) || (e.code == &quot; , &quot;'&quot; , &quot;ArrowDown&quot; , &quot;'&quot; , &quot;) ||
            (e.code == &quot; , &quot;'&quot; , &quot;Delete&quot; , &quot;'&quot; , &quot;) || (e.code == &quot; , &quot;'&quot; , &quot;Backspace&quot; , &quot;'&quot; , &quot;)) {
            return;
        }
        var chr = String.fromCharCode(e.which);
        if (&quot;0123456789., &quot;.indexOf(chr) === -1) {
            return false;
        }
    };
    $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;keypress&quot; , &quot;'&quot; , &quot;, blockNonNumeric);
    $(&quot; , &quot;'&quot; , &quot;#frb-amt-monthly-other-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;keypress&quot; , &quot;'&quot; , &quot;, blockNonNumeric);

};

/**
 * Calculate approximate transaction fee on given amount
 *
 * @param  {number} amount
 * @return {number}        Rounded to 2 decimal places
 */
frb.calculateFee = function(amount) {
    var currency = frb.getCurrency(mw.centralNotice.data.country),
        feeMultiplier = 0.04,
        feeMinimum = frb.amounts.feeMinimums[currency] || 0.35,
        feeAmount = amount * feeMultiplier;

    if ( feeAmount &lt; feeMinimum ) {
      feeAmount = feeMinimum;
    }
    return parseFloat(feeAmount.toFixed(2));
};

frb.updateFeeDisplay = function() {
    var currency = frb.getCurrency(mw.centralNotice.data.country),
        language = mw.centralNotice.data.uselang,
        amount, feeAmount, totalAmount;

    amount = frb.getAmount();
    feeAmount = frb.calculateFee(amount);
    if ( $(&quot; , &quot;'&quot; , &quot;#frb-ptf-checkbox&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;) ) {
        totalAmount = amount + feeAmount;
    } else {
        totalAmount = amount;
    }

    var feeAmountFormatted = frb.formatCurrency(currency, feeAmount, language);
    $(&quot; , &quot;'&quot; , &quot;.frb-ptf-fee&quot; , &quot;'&quot; , &quot;).text(feeAmountFormatted);

    var totalAmountFormatted = frb.formatCurrency(currency, totalAmount, language);
    $(&quot; , &quot;'&quot; , &quot;.frb-ptf-total&quot; , &quot;'&quot; , &quot;).text(totalAmountFormatted);

    $(&quot; , &quot;'&quot; , &quot;.frb-ptf&quot; , &quot;'&quot; , &quot;).slideDown( frb.reduceMotion ? 0 : 400 );
};

/**
 * Custom hide cookie function
 *
 * Purposely sets only for this domain.
 * CentralNotice builtin method seems buggy - see T270401
 *
 * @param {string} reason Reason to store in the hide cookie
 * @param {number} duration Cookie duration, in seconds
 */
frb.altSetHideCookie = function ( reason, duration ) {

    mw.loader.using( &quot; , &quot;'&quot; , &quot;mediawiki.cookie&quot; , &quot;'&quot; , &quot; ).then( function () {

        var cookieName = &quot; , &quot;'&quot; , &quot;centralnotice_hide_fundraising&quot; , &quot;'&quot; , &quot;,
            date = new Date(),
            hideData = {
                v: 1,
                created: Math.floor( date.getTime() / 1000 ),
                reason: reason
            };

        // Re-use the same date object to set the cookie&quot; , &quot;'&quot; , &quot;s expiry time
        date.setSeconds( date.getSeconds() + duration );

        mw.cookie.set(
            cookieName,
            JSON.stringify( hideData ),
            { expires: date, path: &quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;, domain: &quot; , &quot;'&quot; , &quot;wikipedia.org&quot; , &quot;'&quot; , &quot;, prefix: &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; }
        );

    });

};

/**
 * Determine if banner should be shown, and set correct data for impression logging
 *
 * @return {boolean} Show banner?
 */
frb.shouldShowBanner = function() {

    mw.centralNotice.bannerData.hideResult = false;

    /* Hide in unsupported browsers */
    if ( !frb.supportedBrowser ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = &quot; , &quot;'&quot; , &quot;browser&quot; , &quot;'&quot; , &quot;;
    }

    /* Hide outside main namespace (except Main Page, for sites where it isn&quot; , &quot;'&quot; , &quot;t in main namespace) */
    if ( mw.config.get(&quot; , &quot;'&quot; , &quot;wgNamespaceNumber&quot; , &quot;'&quot; , &quot;) > 0 &amp;&amp; !mw.config.get(&quot; , &quot;'&quot; , &quot;wgIsMainPage&quot; , &quot;'&quot; , &quot;) ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = &quot; , &quot;'&quot; , &quot;namespace&quot; , &quot;'&quot; , &quot;;
    }

    // Hide banner on sensitive articles
    // TODO - possibly add wgWikibaseItemId for multilingual support and resilience to moves?
    var hideTitles = [ &quot; , &quot;'&quot; , &quot;Murder of Don Banfield&quot; , &quot;'&quot; , &quot; ];
    if ( hideTitles.indexOf( mw.config.values.wgTitle ) !== -1 ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = &quot; , &quot;'&quot; , &quot;article&quot; , &quot;'&quot; , &quot;;
    }

    /* Hide banner if on wrong site (desktop/mobile) in case wrong device settings were chosen */
    var bannerName = mw.centralNotice.data.banner,
        skin = mw.config.get(&quot; , &quot;'&quot; , &quot;skin&quot; , &quot;'&quot; , &quot;);
    if (
         ( bannerName.indexOf(&quot; , &quot;'&quot; , &quot;_dsk_&quot; , &quot;'&quot; , &quot;) !== -1 &amp;&amp; skin === &quot; , &quot;'&quot; , &quot;minerva&quot; , &quot;'&quot; , &quot; ) ||
         ( bannerName.indexOf(&quot; , &quot;'&quot; , &quot;_m_&quot; , &quot;'&quot; , &quot;) !== -1 &amp;&amp; skin !== &quot; , &quot;'&quot; , &quot;minerva&quot; , &quot;'&quot; , &quot; )
    ) {
        mw.centralNotice.bannerData.hideResult = true;
        mw.centralNotice.bannerData.hideReason = &quot; , &quot;'&quot; , &quot;other&quot; , &quot;'&quot; , &quot;;
        console.warn(&quot; , &quot;'&quot; , &quot;Hiding fundraising banner on wrong site (desktop/mobile)&quot; , &quot;'&quot; , &quot;);
    }

    return !mw.centralNotice.bannerData.hideResult;

};

/* Debug function to highlight dynamically replaced elements */
frb.highlightReplacements = function() {
    $(&quot; , &quot;'&quot; , &quot;.frb [class^=&quot;frb-replace&quot;], .frb-ptf-fee, .frb-ptf-total, .frb-upsell-ask, frb-amt&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;background-color&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#fa0&quot; , &quot;'&quot; , &quot;);
};

if ( !mw.centralNotice.adminUi ) { // T262693
    /**
     * Provides alterImpressionData hook for CentralNotice
     * This info will be sent back with Special:RecordImpression
     * TODO: check if/when we can remove this (and RecordImpression)
     */
    mediaWiki.centralNotice.bannerData.alterImpressionData = function( impressionData ) {
        // Returning true from this function indicates the banner was shown
        if (mediaWiki.centralNotice.bannerData.hideReason) {
            impressionData.reason = mediaWiki.centralNotice.bannerData.hideReason;
        }
        if (mediaWiki.centralNotice.bannerData.cookieCount) {
            impressionData.banner_count = mediaWiki.centralNotice.bannerData.cookieCount;
        }

        return !mediaWiki.centralNotice.bannerData.hideResult;
    };
}

/* End of MediaWiki:FundraisingBanners/CoreJS-2018.js */
/* jshint maxerr: 600 */
/* == MediaWiki:FundraisingBanners/LocalizeJS-2022.js == */

/**
 * Get the currency for a given country
 *
 * NOTE: The following currency mapping is WMF-specific based on payment
 * provider availability, NOT necessarily the official currency of the country
 *
 * @param  {string} country code
 * @return {string} currency code
 */
frb.getCurrency = function(country) {
    switch ( country ) {
        // Big 6
        case &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;CAD&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;AUD&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;NZD&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;GBP&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot;;
        // Euro countries
        case &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot;:
        case &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot;:
            return &quot; , &quot;'&quot; , &quot;EUR&quot; , &quot;'&quot; , &quot;;
        // Others
        case &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;DKK&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;HUF&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;ILS&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;INR&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;JPY&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;MYR&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;NOK&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;PLN&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;CZK&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;RON&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;SEK&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;UAH&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;ZAR&quot; , &quot;'&quot; , &quot;;
        // Latin America
        case &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;BRL&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;ARS&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;CLP&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;COP&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;MXN&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;PEN&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;UYU&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot;;
        case &quot; , &quot;'&quot; , &quot;LI&quot; , &quot;'&quot; , &quot;: return &quot; , &quot;'&quot; , &quot;CHF&quot; , &quot;'&quot; , &quot;;
        // Fall back to USD
        default:
            return &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot;;
    }
};

/**
 * Format a currency value
 *
 * TODO: make this handle the ISO code overrides?
 *
 * @param  {string} currency code. Leave undefined to get without symbol.
 * @param  {number} amount
 * @param  {string} language code
 * @return {string} formatted string e.g. &quot; , &quot;'&quot; , &quot;$3&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;£5&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;10 €&quot; , &quot;'&quot; , &quot;
 */
frb.formatCurrency = function(currency, amount, language) {

    var locale, formatterOptions, formatter, fmAmount, supportsIntl;

    if ( isNaN(amount) || amount === &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ) {
        // Not a number, it&quot; , &quot;'&quot; , &quot;s probably the &quot; , &quot;'&quot; , &quot;other&quot; , &quot;'&quot; , &quot; string or box
        // TODO: better way of doing this?
        fmAmount = amount;
    } else {
        // Check browser support
        try {
            supportsIntl = typeof window.Intl === &quot; , &quot;'&quot; , &quot;object&quot; , &quot;'&quot; , &quot;;
        } catch (e) {
            supportsIntl = false; // T265396
        }

        if ( supportsIntl ) {
            // Use Intl for fancy number formatting - thousands separators etc
            locale = language + &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; + mw.centralNotice.data.country;
            if ( amount % 1 !== 0 ) {
                formatterOptions = { minimumFractionDigits: 2 };
            } else {
                formatterOptions = {};
            }
            formatter = new Intl.NumberFormat(locale, formatterOptions);
        } else {
            // Bad browser i.e. IE. Just do the basics: 2 decimal places if needed, or none
            formatter = {};
            formatter.format = function(number) {
                if ( amount % 1 !== 0 ) {
                    return number.toFixed(2);
                } else {
                    return number.toString();
                }
            };
        }
        fmAmount = formatter.format(amount);
    }

    // No symbol needed
    if ( currency === undefined ) {
        return fmAmount;
    }

    // Better dive into the formatting object
    if ( frb.currencyFormats[currency] === undefined ) {
        return currency + &quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot; + fmAmount;
    }
    if ( frb.currencyFormats[currency] instanceof Object ) { // not a string
        if ( frb.currencyFormats[currency][language] !== undefined ) {
            return frb.currencyFormats[currency][language].replace(&quot; , &quot;'&quot; , &quot;\t&quot; , &quot;'&quot; , &quot;, fmAmount);
        }
        return frb.currencyFormats[currency][&quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot;].replace(&quot; , &quot;'&quot; , &quot;\t&quot; , &quot;'&quot; , &quot;, fmAmount);
    }

    return frb.currencyFormats[currency].replace(&quot; , &quot;'&quot; , &quot;\t&quot; , &quot;'&quot; , &quot;, fmAmount);
};

/*
 * Select the correct amount or array of amounts from object in &quot;source&quot;
 *
 * @param {Object} source   - the amounts data object e.g. frb.amounts.options7, frb.amounts.averages
 * @param {string} currency - ISO code of currency
 * @param {string} country  - ISO code of country (optional)
 * @return {array/number}   - depending on source
 */
frb.pickAmounts = function(source, currency, country) {

    if ( source[currency][&quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot;] ) { // we need to go deeper
        if ( source[currency][country] !== undefined ) {
            return source[currency][country];
        } else {
            return source[currency][&quot; , &quot;'&quot; , &quot;default&quot; , &quot;'&quot; , &quot;];
        }
    } else {
        return source[currency];
    }
};

/* Credit card types so we can show the correct logos */
frb.cardTypes = {
    // Big 6
    &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmad&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    // Euro countries
    &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;, // Adyen
    &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;, // Adyen
    &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    // Others
    &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmad&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmad&quot; , &quot;'&quot; , &quot;, // Adyen
    &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmaj&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;, // Adyen
    &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vm&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vmar&quot; , &quot;'&quot; , &quot;, // dLocal
    &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;,
    &quot; , &quot;'&quot; , &quot;LI&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;vma&quot; , &quot;'&quot; , &quot;
};

/**
 * Should we show Apple Pay?
 *
 * Note there is a ~500ms delay in Safari when checking, so only call this if needed
 *
 * @param  {string} country
 * @return {boolean}
 */
frb.shouldShowApplePay = function ( country ) {
    // https://support.apple.com/en-us/HT207957 - minus China mainland
    var appleCountries = [
        &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TW&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;AM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;EE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FI&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;IS&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;JE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LI&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;MT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MC&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ME&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SM&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;RS&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SI&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VA&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;BH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PS&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;QA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AE&quot; , &quot;'&quot; , &quot;, 
        &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;
    ];
    if ( appleCountries.indexOf( country ) === -1 ) {
        return false;
    }
    if ( location.search.match(&quot; , &quot;'&quot; , &quot;forceApplePay&quot; , &quot;'&quot; , &quot;) ) {
        return true;
    }
    if ( window.ApplePaySession ) {
        if ( ApplePaySession.canMakePayments() ) {
            return true;
        }
    }
    return false;
};

/**
 * Display the correct payment methods for current country
 *
 * Methods should be labeled with class &quot; , &quot;'&quot; , &quot;frb-pm-xxxx&quot; , &quot;'&quot; , &quot;
 * TODO: clean this function up more
 *
 * @param  {string} country
 */
frb.localizeMethods = function(country) {

    // Test country with *all the methods*
    if ( country === &quot; , &quot;'&quot; , &quot;ZZ&quot; , &quot;'&quot; , &quot; ) {
        $(&quot; , &quot;'&quot; , &quot;.frb-payment-options > div&quot; , &quot;'&quot; , &quot;).show();
        return;
    }

    // Hide recurring completely for some countries and endowment
    if ( frb.isEndowment || frb.noRecurringCountries.indexOf(country) !== -1 ) {
        $(&quot; , &quot;'&quot; , &quot;.frb-frequency, .recurring-details&quot; , &quot;'&quot; , &quot;).hide();
    }

    // Remove any leftover WorldPay and Adyen
    $(&quot; , &quot;'&quot; , &quot;.frb-pm-cc-wp&quot; , &quot;'&quot; , &quot;).remove();
    $(&quot; , &quot;'&quot; , &quot;.frb-pm-cc-adyen&quot; , &quot;'&quot; , &quot;).remove();

    // Monthly Adyen credit card is allowed now
    // if ( frb.ccAdyenCountries.indexOf( country ) !== -1 ) {
    //     $(&quot; , &quot;'&quot; , &quot;.frb-pm-cc&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;no-monthly&quot; , &quot;'&quot; , &quot;);
    // }

    // Countries with no PayPal option
    var noPP = [&quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;QA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;OM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BD&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PA&quot; , &quot;'&quot; , &quot;,
                &quot; , &quot;'&quot; , &quot;PY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;JM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DZ&quot; , &quot;'&quot; , &quot;];
    if ($.inArray(country, noPP) !== -1) {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp&quot; , &quot;'&quot; , &quot;).remove();
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp-usd&quot; , &quot;'&quot; , &quot;).remove();
    }

    // Countries with no PayPal for mobile only - https://phabricator.wikimedia.org/T173001
    var noPPmobile = [&quot; , &quot;'&quot; , &quot;PH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ID&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VN&quot; , &quot;'&quot; , &quot;];
    var mobileRegex = /(_mob_|_ipd_|_m_)/;
    if ($.inArray(country, noPPmobile) !== -1) {
        if (mw.centralNotice.data.banner.search(mobileRegex) !== -1) {
            $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp&quot; , &quot;'&quot; , &quot;).remove();
            $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp-usd&quot; , &quot;'&quot; , &quot;).remove();
        }
    }

    // Countries where PayPal must be in USD
    var ppUSD = [&quot; , &quot;'&quot; , &quot;BG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ID&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KR&quot; , &quot;'&quot; , &quot;,
                 &quot; , &quot;'&quot; , &quot;KZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NI&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot;,
                 &quot; , &quot;'&quot; , &quot;BH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IS&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MV&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BZ&quot; , &quot;'&quot; , &quot;,
                 &quot; , &quot;'&quot; , &quot;CR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CW&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;KN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LC&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GD&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FJ&quot; , &quot;'&quot; , &quot;,
                 &quot; , &quot;'&quot; , &quot;TN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BJ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BF&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CI&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GW&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ML&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;,
                 &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;];
    if ($.inArray(country, ppUSD) !== -1) {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp&quot; , &quot;'&quot; , &quot;).remove();
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp-usd&quot; , &quot;'&quot; , &quot;).show();
    } else {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-pp-usd&quot; , &quot;'&quot; , &quot;).remove();
    }

    // Show any extra local payment methods, or remove them if not needed
    var extrapaymentmethods = {
        &quot; , &quot;'&quot; , &quot;amazon&quot; , &quot;'&quot; , &quot;   : [&quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;], // Note Amazon was removed from current best 2023-10-20
        &quot; , &quot;'&quot; , &quot;bpay&quot; , &quot;'&quot; , &quot;     : [],
        &quot; , &quot;'&quot; , &quot;ideal&quot; , &quot;'&quot; , &quot;    : [&quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;],
        &quot; , &quot;'&quot; , &quot;bt&quot; , &quot;'&quot; , &quot;       : [&quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;], // Bank Transfer (dLocal/Adyen)
        &quot; , &quot;'&quot; , &quot;cash&quot; , &quot;'&quot; , &quot;     : [&quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot;],  // &quot; , &quot;'&quot; , &quot;Cash&quot; , &quot;'&quot; , &quot; methods (dLocal)
        &quot; , &quot;'&quot; , &quot;pix&quot; , &quot;'&quot; , &quot;      : [&quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;],
        &quot; , &quot;'&quot; , &quot;boleto&quot; , &quot;'&quot; , &quot;   : [&quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;]
    };

    // Methods with different labels per country

    var language = mw.config.get(&quot; , &quot;'&quot; , &quot;wgUserLanguage&quot; , &quot;'&quot; , &quot;);
    var btTranslation = &quot; , &quot;'&quot; , &quot;Bank Transfer&quot; , &quot;'&quot; , &quot;;

    if (language === &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot;) {

        if (country === &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;) {
            btTranslation = &quot; , &quot;'&quot; , &quot;Transferência bancária&quot; , &quot;'&quot; , &quot;;
        }

    } else if (language === &quot; , &quot;'&quot; , &quot;es&quot; , &quot;'&quot; , &quot;) {

        if (country === &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;) {
            btTranslation = &quot; , &quot;'&quot; , &quot;WebPay&quot; , &quot;'&quot; , &quot;;
        } else if (country === &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;) {
            btTranslation = &quot; , &quot;'&quot; , &quot;PSE Pagos&quot; , &quot;'&quot; , &quot;;
        } else {
            btTranslation = &quot; , &quot;'&quot; , &quot;Transferencia bancaria&quot; , &quot;'&quot; , &quot;;
        }

    }

    if (country === &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;) {
        if (language === &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;) {
            btTranslation = &quot; , &quot;'&quot; , &quot;Online Banking&quot; , &quot;'&quot; , &quot;;
        }
        if (language === &quot; , &quot;'&quot; , &quot;cs&quot; , &quot;'&quot; , &quot;) {
            btTranslation = &quot; , &quot;'&quot; , &quot;Internetové Bankovnictví&quot; , &quot;'&quot; , &quot;;
        }
    }

    $( &quot; , &quot;'&quot; , &quot;.frb-pm-bt button, .frb-pm-bt label, button.frb-pm-bt&quot; , &quot;'&quot; , &quot; ).text( btTranslation );

    for (var method in extrapaymentmethods) {
        var $methodbutton = $(&quot; , &quot;'&quot; , &quot;.frb-pm-&quot; , &quot;'&quot; , &quot; + method);
        if ( $.inArray(country, extrapaymentmethods[method]) !== -1 &amp;&amp; !frb.isEndowment ) {
            $methodbutton.show();
        } else {
            $methodbutton.remove();
        }
    }

    // Google Pay - separated from extrapaymentmethods as we want to show on Endowment too
    var googlePayCountries = [
        &quot; , &quot;'&quot; , &quot;AE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;CZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;DK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;EE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ES&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;HR&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;HU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;JP&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;LV&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;NO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;OM&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PT&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;QA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;RU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SE&quot; , &quot;'&quot; , &quot;,
        &quot; , &quot;'&quot; , &quot;SG&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;SK&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TH&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;TW&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;VN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ZA&quot; , &quot;'&quot; , &quot;
    ];
    if ( $.inArray(country, googlePayCountries) !== -1 ) {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-google&quot; , &quot;'&quot; , &quot;).show();
    } else {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-google&quot; , &quot;'&quot; , &quot;).remove();
    }

    // Apple Pay
    if ( $(&quot; , &quot;'&quot; , &quot;.frb-pm-applepay&quot; , &quot;'&quot; , &quot;).length > 0 ) {
        if ( !frb.shouldShowApplePay( country ) ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-pm-applepay&quot; , &quot;'&quot; , &quot;).remove();
        }
    }

	// Venmo
	var $venmo = $(&quot; , &quot;'&quot; , &quot;.frb-pm-venmo&quot; , &quot;'&quot; , &quot;);
	if ( country === &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; &amp;&amp; $venmo.length > 0 ) {
		// From MediaWiki:FundraisingBanners/VenmoBrowserCheck.js
		if ( frb.isVenmoSupported() ) {
			$venmo.show();
		} else {
			$venmo.remove();
		}
	} else {
		$venmo.remove();
	}

    /* Add card types class to credit card button, so we can show correct logos */
    if ( frb.cardTypes[country] ) {
        $(&quot; , &quot;'&quot; , &quot;.frb-pm-cc&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-cctypes-&quot; , &quot;'&quot; , &quot; + frb.cardTypes[country] );
    }
};

/**
 * Check scheduled payment method outages and hide buttons if needed
 *
 * Data at https://meta.wikimedia.org/wiki/MediaWiki:FR2013/Resources/PaymentOutages.js
 * Methods should be labeled with class &quot; , &quot;'&quot; , &quot;frb-pm-xxxx&quot; , &quot;'&quot; , &quot;
 *
 * @param  {string} country code
 */
frb.checkMethodOutages = function(country) {

    // TODO - can we load this a better way?
    /* This file can be used to schedule hiding of individual payment methods from banners
 * e.g. if they have scheduled downtime.
 *
 * Valid methods are:
 *	ideal, cc, pp, amazon, bpay, webmoney, cash, pp-usd
 * (most of the time it&quot; , &quot;'&quot; , &quot;s &quot; , &quot;'&quot; , &quot;ideal&quot; , &quot;'&quot; , &quot;...)
 * Can also limit outage to a specific country with country: &quot;XX&quot; (where XX is an ISO code)
 *
 * Note that in JavaScript dates the months (and only the months) start at 0.
 * Jan=0, Feb=1, Mar=2, Apr=3 etc. How hateful.
 *
 * Be sure to also update donatewiki if needed e.g. by commenting the method templates
 * found at https://donate.wikimedia.org/wiki/Template:2012FR/Form-section/Paymentmethods
 * 
 */
var outages = [
    {
        start:      new Date(Date.UTC(2016, 8, 18, 1)),
        end:        new Date(Date.UTC(2016, 8, 18, 7)),
        method:     &quot;ideal&quot;
    }
]; // jshint ignore:line
    var now = new Date();

    for (var i = outages.length - 1; i >= 0; i--) {
        if ( now > outages[i].start &amp;&amp; now &lt; outages[i].end ) {
            if (outages[i].country === undefined || outages[i].country == country) {
                $(&quot; , &quot;'&quot; , &quot;.frb-pm-&quot; , &quot;'&quot; , &quot; + outages[i].method).hide();
            }
        }
    }
};

/**
 * Adjust the amount options and their labels
 *
 * Inputs should have id frb-amt-psX where X is the index number (starting from 1)
 *
 * @param  {Object}  source     - object with amounts e.g. frb.amounts.options7
 * @param  {string}  currency   - currency code e.g. &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot;
 * @param  {string}  country    - country code  e.g. &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot; Some currencies can have different options per country.
 * @param  {string}  language   - language code e.g. &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; For symbol formatting
 * @param  {boolean} useSymbols - use currency symbols on labels or not? (3 vs $3)
 */
frb.localizeAmountOptions = function(source, currency, country, language, useSymbols) {

    var amountOptions = frb.pickAmounts(source, currency, country);

    $(&quot; , &quot;'&quot; , &quot;#frb-form input[name=&quot;amount&quot;]&quot; , &quot;'&quot; , &quot;).each(function(index) {
        var $input = $(this);
        var $label = $input.siblings(&quot; , &quot;'&quot; , &quot;label&quot; , &quot;'&quot; , &quot;);

        var i = $input.attr(&quot; , &quot;'&quot; , &quot;id&quot; , &quot;'&quot; , &quot;).replace(&quot; , &quot;'&quot; , &quot;frb-amt-ps&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
        var amount = amountOptions[i-1]; // because IDs start from 1

        if ( amount ) {
            $input.val( amount );
            if ( useSymbols ) {
                $label.text( frb.formatCurrency( currency, amount, language) );
            } else {
                $label.text( frb.formatCurrency( undefined, amount, language) );
            }
        }
    });

};

/**
 * Make an element into a link
 *
 * @param  {string} selector    CSS selector for elements to convert to a link
 * @param  {string} language    Code of language (could be es-419 or pt-br)
 * @param  {string} baseUrl     URL of link (function will add language parameter)
 */
frb.makeLink = function( selector, language, baseUrl ) {
    var url = baseUrl + &quot; , &quot;'&quot; , &quot;&amp;language=&quot; , &quot;'&quot; , &quot; + language;
    $( selector ).each( function() {
        var $link = $( &quot; , &quot;'&quot; , &quot;&lt;a>&lt;/a>&quot; , &quot;'&quot; , &quot; );
        $link.html( $( this ).html() );
        $link.attr( { href: url, target: &quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot; } );
        $( this ).replaceWith( $link );
    });
};

/**
 * Get the number of banners seen from localStorage
 * @return {number} Number of banners seen
 */
frb.getSeenCount = function () {

    // Force with URL parameter &quot; , &quot;'&quot; , &quot;impression&quot; , &quot;'&quot; , &quot;
    if ( typeof URLSearchParams === &quot; , &quot;'&quot; , &quot;function&quot; , &quot;'&quot; , &quot; ) { // not available in old browsers
        var urlParams = new URLSearchParams( window.location.search );
        if ( urlParams.has( &quot; , &quot;'&quot; , &quot;impression&quot; , &quot;'&quot; , &quot; ) ) {
            return urlParams.get( &quot; , &quot;'&quot; , &quot;impression&quot; , &quot;'&quot; , &quot; );
        }
    }

    try {
        if ( localStorage ) {
            var identifier = mw.centralNotice.internal.state.campaign.mixins.impressionDiet.cookieName,
                lsName = &quot; , &quot;'&quot; , &quot;CentralNoticeKV|global|impression_diet_&quot; , &quot;'&quot; , &quot; + identifier,
                diet = JSON.parse( localStorage.getItem( lsName ) );
            if ( diet ) {
                return diet.val.seenCount;
            }
        }
    } catch ( ex ) {
        // do nothing - localStorage is configured not to let us read it, or mixin not set
        return;
    }
};

/**
 * Helper function to do text replacements and wrap them in correct class
 * 
 * @param  {RegExp} regex       Regular expression to replace
 * @param  {string} replacement String to replace it with
 */
frb.textReplace = function( regex, replacement ) {
    $( &quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot; ).each( function( index ) {
        var newHtml = $( this ).html();
        newHtml = newHtml.replace( regex, &quot; , &quot;'&quot; , &quot;&lt;span class=&quot;frb-replaced&quot;>&quot; , &quot;'&quot; , &quot; + replacement + &quot; , &quot;'&quot; , &quot;&lt;/span>&quot; , &quot;'&quot; , &quot; );
        $( this ).html( newHtml );
    });
};

/**
 * Replace elements with preset ask string amounts
 *
 * e.g. class=&quot;frb-replace-amt-ps4&quot; will be replaced with amount #4, currently $25 in the US
 *
 * @param  {string} currency - currency code e.g. &quot; , &quot;'&quot; , &quot;USD&quot; , &quot;'&quot; , &quot;
 * @param  {string} country  - country code  e.g. &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;
 * @param  {string} language - language code e.g. &quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot; For symbol formatting
 */
frb.replaceCustomAmounts = function( currency, country, language ) {
    var amountOptions = frb.pickAmounts( frb.amounts.options7, currency, country );

    // Old style element replacements
    $( &quot; , &quot;'&quot; , &quot;.frb [class^=&quot;frb-replace-amt-ps&quot;]&quot; , &quot;'&quot; , &quot; ).each( function() {
        var i = $( this ).attr( &quot; , &quot;'&quot; , &quot;class&quot; , &quot;'&quot; , &quot; ).replace( &quot; , &quot;'&quot; , &quot;frb-replace-amt-ps&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ),
            amount = amountOptions[ i - 1 ],
            formattedAmount = frb.formatCurrency( currency, amount, language );
        $( this ).html( &quot; , &quot;'&quot; , &quot;&lt;frb-amt>&quot; , &quot;'&quot; , &quot; + formattedAmount + &quot; , &quot;'&quot; , &quot;&lt;/frb-amt>&quot; , &quot;'&quot; , &quot; );
    });

    // Text replacements e.g. %amount-4%
    // There is probably a more efficient way to do this, but it&quot; , &quot;'&quot; , &quot;s at least fairly simple
    for (var i = 0; i &lt; amountOptions.length; i++) {
        var amount = amountOptions[i],
            formattedAmount,
            regex = new RegExp( &quot; , &quot;'&quot; , &quot;%amount-&quot; , &quot;'&quot; , &quot; + (i+1) + &quot; , &quot;'&quot; , &quot;%&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;gi&quot; , &quot;'&quot; , &quot; );
        if ( frb.textAmountIsoCountries.includes( country ) ) {
            formattedAmount = frb.formatCurrency( undefined, amount, language ) + &quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot; + currency;
        } else {
            formattedAmount = frb.formatCurrency( currency, amount, language );
        }
        frb.textReplace( regex, formattedAmount );
    }
};

/**
 * Get today&quot; , &quot;'&quot; , &quot;s date like &quot;December 3&quot; - English only for now
 * 
 * @return {string} Today&quot; , &quot;'&quot; , &quot;s date as a string
 */
frb.getDateString = function() {
    var date = new Date(),
        locale = mw.centralNotice.data.uselang + &quot; , &quot;'&quot; , &quot;-&quot; , &quot;'&quot; , &quot; + mw.centralNotice.data.country;
    return date.toLocaleString( locale, { day: &quot; , &quot;'&quot; , &quot;numeric&quot; , &quot;'&quot; , &quot;, month: &quot; , &quot;'&quot; , &quot;long&quot; , &quot;'&quot; , &quot; } );
};

frb.noRecurringCountries = [&quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;];
frb.ccAdyenCountries     = [&quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA&quot; , &quot;'&quot; , &quot;];

/* These countries use potentially ambiguous $ sign.
Use ISO code instead in text (but still $ for buttons) */
frb.textAmountIsoCountries = [&quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;];

$(function() {

    if ( mw.centralNotice.adminUi ) { // T262693
        return;
    }

    var language = mw.centralNotice.data.uselang;
    var variantLanguage; // for pt-br and es-419, note we can only use these for certain links
    var country  = mw.centralNotice.data.country;
    var currency = frb.getCurrency(country);

    if ( language === &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot; &amp;&amp; country === &quot; , &quot;'&quot; , &quot;BR&quot; , &quot;'&quot; , &quot; ) {
        variantLanguage = &quot; , &quot;'&quot; , &quot;pt-br&quot; , &quot;'&quot; , &quot;;
    } else if ( language === &quot; , &quot;'&quot; , &quot;es&quot; , &quot;'&quot; , &quot; &amp;&amp; [&quot; , &quot;'&quot; , &quot;AR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CL&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CO&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;PE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;MX&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UY&quot; , &quot;'&quot; , &quot;].indexOf( country ) !== -1 ) {
        variantLanguage = &quot; , &quot;'&quot; , &quot;es-419&quot; , &quot;'&quot; , &quot;;
    } else {
        variantLanguage = language;
    }

    // Payment methods
    frb.localizeMethods(country);
    frb.checkMethodOutages(country);

    // Preset amounts
    frb.replaceCustomAmounts( currency, country, language );

    // Basic replacements
    $(&quot; , &quot;'&quot; , &quot;.frb-replace-currencysymbol&quot; , &quot;'&quot; , &quot;).text( frb.formatCurrency( currency, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;, language ).replace(&quot; , &quot;'&quot; , &quot; &quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) );
    $(&quot; , &quot;'&quot; , &quot;.frb-replace-currencycode&quot; , &quot;'&quot; , &quot;).text( currency );

    // Country name
    var countryName;
    if ( frb.countryNames[language] ) {
        countryName = frb.countryNames[language][country] || frb.countryNames.en[country];
    } else {
        countryName = frb.countryNames.en[country];
    }
    $( &quot; , &quot;'&quot; , &quot;.frb-replace-countryname&quot; , &quot;'&quot; , &quot; ).text( countryName );
    frb.textReplace( /%country%/gi, countryName );

    // &quot;in COUNTRY&quot; or equivalent
    var inCountryName;
    if ( frb.inCountryNames[language] ) {
        inCountryName = frb.inCountryNames[language][country] || frb.inCountryNames.en[country];
    } else {
        inCountryName = frb.inCountryNames.en[country];
    }
    $( &quot; , &quot;'&quot; , &quot;.frb-replace-incountryname&quot; , &quot;'&quot; , &quot; ).text( inCountryName );
    frb.textReplace( /%in-country%/gi, inCountryName );

    // Day of week
    // TODO: Replace these with date.toLocaleString so we can drop frb.dayNames? 
    //       Might still need some ways to deal with &quot;this&quot; and capitalization
    var now = new Date();
    var dayNumber = now.getDay();
    var capitalizeText = function( text ) {
        // Capitalize first letter, for use at start of sentence
        return text.charAt(0).toUpperCase() + text.slice(1);
    };

    if ( $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek, .frb-replace-dayofweek-capitalize&quot; , &quot;'&quot; , &quot;).length > 0 ) {
        if ( frb.dayNames[language] ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek&quot; , &quot;'&quot; , &quot;).text( frb.dayNames[language][dayNumber] );
            $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek-capitalize&quot; , &quot;'&quot; , &quot;).text( capitalizeText( frb.dayNames[language][dayNumber] ) );
        } else {
            console.log(&quot; , &quot;'&quot; , &quot;Warning: banner should contain a day of the week, but no translations found.&quot; , &quot;'&quot; , &quot;);
        }
    }

    if ( $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek-this, .frb-replace-dayofweek-this-capitalize&quot; , &quot;'&quot; , &quot;).length > 0 ) {
        if ( frb.dayNamesThis[language] ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek-this&quot; , &quot;'&quot; , &quot;).text( frb.dayNamesThis[language][dayNumber] );
            $(&quot; , &quot;'&quot; , &quot;.frb-replace-dayofweek-this-capitalize&quot; , &quot;'&quot; , &quot;).text( capitalizeText( frb.dayNamesThis[language][dayNumber] ) );
        } else {
            console.log(&quot; , &quot;'&quot; , &quot;Warning: banner should contain &quot;this DAY&quot;, but no translations found.&quot; , &quot;'&quot; , &quot;);
        }
    }

    // Simple %weekday% text replacement
    try {
        if ( frb.dayNames[language] ) {
            frb.textReplace( /%weekday%/gi, frb.dayNames[language][dayNumber] );
        } else {
            frb.textReplace( /%weekday%/gi, frb.dayNames[&quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;][dayNumber] );
        }
    } catch ( error ) {
        console.error( error );
    }

    // Replace %date% with today&quot; , &quot;'&quot; , &quot;s date e.g. &quot;December 3&quot;
    try {
        frb.textReplace( /%date%/gi, frb.getDateString() );
    } catch ( error ) {
    	console.log( error );
    }

    // Capitalize
    $(&quot; , &quot;'&quot; , &quot;.frb-capitalize&quot; , &quot;'&quot; , &quot;).text(function( index, text ) {
        return text.charAt(0).toUpperCase() + text.slice(1);
    });

    // Replace %average%, %minimum% and %amount%
    var average = frb.pickAmounts( frb.amounts.averages, currency, country ),
        ifEveryone = frb.pickAmounts( frb.amounts.ifEveryone, currency, country ),
        avgString,
        ifString;

    if ( frb.textAmountIsoCountries.indexOf(country) !== -1 ) {
        avgString = frb.formatCurrency( undefined, average, language ) + &quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot; + currency;
        ifString  = frb.formatCurrency( undefined, ifEveryone, language ) + &quot; , &quot;'&quot; , &quot;&amp;nbsp;&quot; , &quot;'&quot; , &quot; + currency;
    } else {
        avgString = frb.formatCurrency( currency, average, language ).replace( /\.$/, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; ); // strip any period from end for use in running text
        ifString  = frb.formatCurrency( currency, ifEveryone, language ).replace( /\.$/, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot; );
    }
    frb.textReplace( /%average%/gi, avgString );
    frb.textReplace( /%minimum%/gi, ifString );
    frb.textReplace( /%amount%/gi,  ifString );

    /**
     * Call a function on every text node contained by a root node.
     *
     * Used so we can do text replacements without accidentally clobbering html and scripts
     *
     * @param  {Node}     rootNode The Node object whose descendants will be recursed through
     * @param  {Function} callback Callback function that receives a Node as its only argument
     */
    function eachTextNode( rootNode, callback ) {
        for ( var node = rootNode.firstChild; node !== null; node = node.nextSibling ) {
            if ( node.nodeType === Node.TEXT_NODE ) {
                callback( node );
            } else if ( node.nodeType === Node.ELEMENT_NODE ) {
                eachTextNode( node, callback );
            }
        }
    }

    // French spacing: replace space before punctuation with &amp;nbsp;
    if ( language === &quot; , &quot;'&quot; , &quot;fr&quot; , &quot;'&quot; , &quot; ) {
        var bannerRootElements = document.getElementsByClassName( &quot; , &quot;'&quot; , &quot;frb&quot; , &quot;'&quot; , &quot; );
        for ( var i = 0; i &lt; bannerRootElements.length; i++ ) {
            eachTextNode( bannerRootElements[i], function( node ) {
                node.textContent = node.textContent.replace( / ([!?;:%])/g, &quot; , &quot;'&quot; , &quot;\u00a0$1&quot; , &quot;'&quot; , &quot; );
            });
        }
    }

    // Links (in smallprint) TODO: merge with frb.makeLink()
    $(&quot; , &quot;'&quot; , &quot;.frb-localize-links a&quot; , &quot;'&quot; , &quot;).each(function() {
        // Add parameters for LandingCheck
        var uri = new mw.Uri( $(this).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;) );
        uri.extend({
            country:      country,
            language:     variantLanguage,
            uselang:      variantLanguage,
            utm_medium:   &quot; , &quot;'&quot; , &quot;sitenotice&quot; , &quot;'&quot; , &quot;,
            utm_campaign: mw.centralNotice.data.campaign || &quot; , &quot;'&quot; , &quot;test&quot; , &quot;'&quot; , &quot;,
            utm_source:   mw.centralNotice.data.banner
        });
        $(this).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, uri.toString());
        $(this).attr(&quot; , &quot;'&quot; , &quot;target&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;_blank&quot; , &quot;'&quot; , &quot;); // Make links open in new tab
    });

    // Add links
    frb.makeLink( &quot; , &quot;'&quot; , &quot;.frb-link-privacy&quot; , &quot;'&quot; , &quot;, variantLanguage, &quot; , &quot;'&quot; , &quot;https://foundation.wikimedia.org/wiki/Special:LandingCheck?basic=true&amp;landing_page=Donor_privacy_policy&quot; , &quot;'&quot; , &quot; );
    frb.makeLink( &quot; , &quot;'&quot; , &quot;.frb-link-tax&quot; , &quot;'&quot; , &quot;,     variantLanguage, &quot; , &quot;'&quot; , &quot;https://donate.wikimedia.org/wiki/Special:LandingCheck?basic=true&amp;landing_page=Tax_deductibility&quot; , &quot;'&quot; , &quot; );
    frb.makeLink( &quot; , &quot;'&quot; , &quot;.frb-link-cancel&quot; , &quot;'&quot; , &quot;,  variantLanguage, &quot; , &quot;'&quot; , &quot;https://donate.wikimedia.org/wiki/Special:LandingCheck?basic=true&amp;landing_page=Cancel_or_change_recurring_giving&quot; , &quot;'&quot; , &quot; );

    // Legal text variants
    if (country === &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;) {
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-US&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-nonUS, .frb-legal-NL&quot; , &quot;'&quot; , &quot;).hide();
    } else if (country === &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;) {
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-NL&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-US, .frb-legal-nonUS&quot; , &quot;'&quot; , &quot;).hide();
    } else {
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-nonUS&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;.frb-legal-US, .frb-legal-NL&quot; , &quot;'&quot; , &quot;).hide();
    }

    // Quick hack for American/British/Commonwealth English differences
    if ( country === &quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot; ) {
        $(&quot; , &quot;'&quot; , &quot;.frb-lang-enUS&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;.frb-lang-en5C&quot; , &quot;'&quot; , &quot;).hide();
    } else {
        $(&quot; , &quot;'&quot; , &quot;.frb-lang-enUS&quot; , &quot;'&quot; , &quot;).hide();
        $(&quot; , &quot;'&quot; , &quot;.frb-lang-en5C&quot; , &quot;'&quot; , &quot;).show();
    }

    // Add this so they get white-space: nowrap from CSS
    $(&quot; , &quot;'&quot; , &quot;.frb-ptf-fee, .frb-ptf-total, .frb-upsell-ask&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-replaced&quot; , &quot;'&quot; , &quot;);

    // Where Remind Me Later should be shown
    var rmlCountries = [&quot; , &quot;'&quot; , &quot;US&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;CA&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;GB&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;IE&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;AU&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NZ&quot; , &quot;'&quot; , &quot;,
                        &quot; , &quot;'&quot; , &quot;IN&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;FR&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;NL&quot; , &quot;'&quot; , &quot;];
    var rmlLanguages = [&quot; , &quot;'&quot; , &quot;en&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;nl&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;ja&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;it&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;sv&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;pt&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;cs&quot; , &quot;'&quot; , &quot;];
    var rmlEnabled = !frb.isEndowment &amp;&amp; rmlCountries.indexOf(country) !== -1 &amp;&amp; rmlLanguages.indexOf(language) !== -1;

    if ( rmlEnabled ) {
        $(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-rml-enabled&quot; , &quot;'&quot; , &quot;);
    } else {
        $(&quot; , &quot;'&quot; , &quot;.frb&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-rml-disabled&quot; , &quot;'&quot; , &quot;);
    }

});

/* == end of MediaWiki:FundraisingBanners/LocalizeJS-2022.js == */
/* eslint-env es6 */
var frb = frb || {};

/* Based on github:braintree/braintree-web/src/venmo/shared/supports-venmo.js */
frb.isVenmoSupported = function(options) {
  var options = options || {
    allowNewBrowserTab: false,
    allowWebviews: true,
    allowDesktop: true,
    allowDesktopWebLogin: true
  };
  var ua = window.navigator.userAgent;

  var merchantAllowsReturningToNewBrowserTab,
    merchantAllowsWebviews,
    merchantAllowsDesktopBrowsers;
  var isMobileDevice = isAndroid() || isIos();
  var isAndroidChrome = isAndroid() &amp;&amp; isChrome();
  var isMobileDeviceThatSupportsReturnToSameTab = isIosSafari() || isAndroidChrome;
  var isKnownUnsupportedMobileBrowser = isIosChrome() || isFacebookOwnedBrowserOnAndroid() || isSamsung();

  options = options || {};
  // NEXT_MAJOR_VERSION allowDesktop will default to true, but can be opted out
  merchantAllowsDesktopBrowsers =
    (options.allowDesktopWebLogin || options.allowDesktop) === true;
  merchantAllowsReturningToNewBrowserTab = options.hasOwnProperty(
    &quot;allowNewBrowserTab&quot;
  )
    ? options.allowNewBrowserTab
    : true;
  // NEXT_MAJOR_VERSION webviews are not supported, except for the case where
  // the merchant themselves is presenting venmo in a webview using the deep
  // link url to get back to their app. For the next major version, we should
  // just not have this option and instead require the merchant to determine
  // if the venmo button should be displayed when presenting it in the
  // merchant&quot; , &quot;'&quot; , &quot;s app via a webview.
  merchantAllowsWebviews = options.hasOwnProperty(&quot;allowWebviews&quot;)
    ? options.allowWebviews
    : true;

  if (isKnownUnsupportedMobileBrowser) {
    return false;
  }

  if (
    !merchantAllowsWebviews &amp;&amp;
    (isAndroidWebview() || isIosWebview())
  ) {
    return false;
  }

  if (!isMobileDevice) {
    return merchantAllowsDesktopBrowsers;
  }

  if (!merchantAllowsReturningToNewBrowserTab) {
    return isMobileDeviceThatSupportsReturnToSameTab;
  }

  return isMobileDevice;

  /* -- functions mostly from github:braintree/browser-detection library -- */

  function isAndroid() {
    return /Android/i.test(ua);
  }

  function isIos(checkIpadOS = true) {
    const iOsTest = /iPhone|iPod|iPad/i.test(ua);
    return checkIpadOS ? iOsTest || isIpadOS() : iOsTest;
  }

  function isIpadOS() {
    // &quot;ontouchend&quot; is used to determine if a browser is on an iPad, otherwise
    // user-agents for iPadOS behave/identify as a desktop browser
    return /Mac|iPad/i.test(ua) &amp;&amp; &quot;ontouchend&quot; in window.document;
  }

  function isEdge() {
    return ua.indexOf(&quot;Edge/&quot;) !== -1 || ua.indexOf(&quot;Edg/&quot;) !== -1;
  }

  function isSamsung() {
    return /SamsungBrowser/i.test(ua);
  }

  function isDuckDuckGo() {
    return ua.indexOf(&quot;DuckDuckGo/&quot;) !== -1;
  }

  function isOpera() {
    return (
      ua.indexOf(&quot;OPR/&quot;) !== -1 ||
      ua.indexOf(&quot;Opera/&quot;) !== -1 ||
      ua.indexOf(&quot;OPT/&quot;) !== -1
    );
  }

  function isSilk() {
    return ua.indexOf(&quot;Silk/&quot;) !== -1;
  }

  function isChrome() {
    return (
      (ua.indexOf(&quot;Chrome&quot;) !== -1 || ua.indexOf(&quot;CriOS&quot;) !== -1) &amp;&amp;
      !isEdge() &amp;&amp;
      !isSamsung() &amp;&amp;
      !isDuckDuckGo() &amp;&amp;
      !isOpera() &amp;&amp;
      !isSilk()
    );
  }

  function isIosFirefox() {
    return /FxiOS/i.test(ua);
  }

  function isWebkit() {
    const webkitRegexp = /webkit/i;
    return webkitRegexp.test(ua);
  }

  function isIosChrome() {
    return ua.indexOf(&quot;CriOS&quot;) > -1;
  }

  function isFacebook() {
    return ua.indexOf(&quot;FBAN&quot;) > -1;
  }

  function isIosSafari() {
    return (
      isIos() &amp;&amp;
      isWebkit() &amp;&amp;
      !isIosChrome() &amp;&amp;
      !isIosFirefox() &amp;&amp;
      !isFacebook()
    );
  }

  function isFacebookOwnedBrowserOnAndroid() {
    var e = ua.toLowerCase();
    return -1 &lt; e.indexOf(&quot;huawei&quot;) &amp;&amp; -1 &lt; e.indexOf(&quot;fban&quot;) || isAndroid() &amp;&amp; (-1 &lt; e.indexOf(&quot;fb_iab&quot;) || -1 &lt; e.indexOf(&quot;instagram&quot;));
  }

  function isSamsungBrowser() {
    return /SamsungBrowser/i.test(ua);
  }

  function isAndroidWebview() {
    return isAndroid() &amp;&amp; -1 &lt; ua.toLowerCase().indexOf(&quot;wv&quot;);
  }

  function isGoogleSearchApp() {
    return /\bGSA\b/.test(ua);
  }

  function isIosGoogleSearchApp() {
    return isIos() &amp;&amp; isGoogleSearchApp();
  }

  function isIosWebview() {
    if (isIos()) {
      // The Google Search iOS app is technically a webview and doesn&quot; , &quot;'&quot; , &quot;t support popups.
      if (isIosGoogleSearchApp()) {
        return true;
      }
      // Historically, a webview could be identified by the presence of AppleWebKit and _no_ presence of Safari after.
      return /.+AppleWebKit(?!.*Safari)/i.test(ua);
    }
    return false;
  }
};

$(function() {

    if ( mw.centralNotice.adminUi || !frb.supportedBrowser ) { // T262693
        return;
    }

    var language = mw.centralNotice.data.uselang;
    var country  = mw.centralNotice.data.country;
    var currency = frb.getCurrency(country);
    var validAmount;
    var validMethod;
    var validOptin;
    var form = document.getElementById(&quot; , &quot;'&quot; , &quot;frb-form&quot; , &quot;'&quot; , &quot;);

    var animationDuration = frb.reduceMotion ? 0 : 400;

    mw.loader.using([&quot; , &quot;'&quot; , &quot;mediawiki.util&quot; , &quot;'&quot; , &quot;]).then(function() {
        frb.rml.init();
    });

    frb.initAmountOptions();
    frb.localizeAmountOptions( frb.amounts.options7, currency, country, language, true );
    frb.localizeErrors();

    frb.storedOptions = {};
    frb.extraData = {};

    frb.setMethod = function (options, frequency) {
        frb.storedOptions = options;

        if( frequency === &quot; , &quot;'&quot; , &quot;no-monthly&quot; , &quot;'&quot; , &quot; ) {
            $(&quot; , &quot;'&quot; , &quot;#frb-frequency-monthly&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, true);
        } else {
            $(&quot; , &quot;'&quot; , &quot;#frb-frequency-monthly&quot; , &quot;'&quot; , &quot;).attr(&quot; , &quot;'&quot; , &quot;disabled&quot; , &quot;'&quot; , &quot;, false);
        }
    };

    frb.updateUpsellAsk = function(isOtherAmountStep) {
        var amount, feeAmount, upsellAmount,
            list = frb.amounts.monthlySuggest[currency];

        if ( list === undefined ) {
            console.log(&quot; , &quot;'&quot; , &quot;No monthlySuggest amounts found for &quot; , &quot;'&quot; , &quot; + currency);
            return;
        }

        // If user is on third step (write a different amount) then get monthly amount if not, the the first form amount
        if (isOtherAmountStep !== undefined) {
            amount = frb.getMonthlyAmount();
        } else {
            amount = frb.getAmount(form);
        }

        // If PTF is checked when we need to calculate the fee for that amount
        if ( $(&quot; , &quot;'&quot; , &quot;#frb-ptf-checkbox&quot; , &quot;'&quot; , &quot;).prop(&quot; , &quot;'&quot; , &quot;checked&quot; , &quot;'&quot; , &quot;) ) {
            amount = amount + frb.calculateFee(amount);
        }

        for (var i = list.length - 1; i >= 0; i--) {
            if ( amount &lt;= list[i][0] ) {
                upsellAmount = list[i][1];
            }
        }

        // If user is in the upsell (second step) then the form.otherMonthlyAmount.value will be updated with the upsellAmount calculated
        if (isOtherAmountStep === undefined) {
            form.otherMonthlyAmount.value = upsellAmount;
        }

        // A formatted value will be returned
        var upsellAmountFormatted = frb.formatCurrency(currency, upsellAmount, language);

        // The value of the amount will be updated only if the user is in the upsell (second step)
        if (isOtherAmountStep === undefined) {
            $(&quot; , &quot;'&quot; , &quot;.frb-upsell-ask&quot; , &quot;'&quot; , &quot;).text(upsellAmountFormatted);
        }
    };

    $(&quot; , &quot;'&quot; , &quot;.frb-amounts input:not(#input_amount_other)&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input change&quot; , &quot;'&quot; , &quot;, function(e) {

        // Deal with https://phabricator.wikimedia.org/T191417
        if ( this.value === &quot;&quot; ) {
            return;
        }

        if ( frb.validateAmount() ) {
            validAmount = 1;
        } else {
            validAmount = 0;
        }
        frb.updateFeeDisplay();
        frb.activateCTA();
    });

    $(&quot; , &quot;'&quot; , &quot;.frb-methods&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        $(&quot; , &quot;'&quot; , &quot;.frb-methods&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-method&quot; , &quot;'&quot; , &quot;).hide();
        $(&quot; , &quot;'&quot; , &quot;.frb-optin&quot; , &quot;'&quot; , &quot;).slideDown( animationDuration );
        validMethod = 1;
        frb.activateCTA();
    });

    // Opt-in interaction
    $(&quot; , &quot;'&quot; , &quot;.frb-optin&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;change&quot; , &quot;'&quot; , &quot;, function() {
        $(&quot; , &quot;'&quot; , &quot;.frb-optin&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.frb-error-optin&quot; , &quot;'&quot; , &quot;).hide();
        if ( $(&quot; , &quot;'&quot; , &quot;#frb-optin-no&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;) ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-optin-no-prompt&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;is-positive&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-optin-no-prompt&quot; , &quot;'&quot; , &quot;).slideDown( animationDuration );
        } else {
            $(&quot; , &quot;'&quot; , &quot;.frb-optin-no-prompt&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;is-positive&quot; , &quot;'&quot; , &quot;);
        }
        validOptin = 1;
        frb.activateCTA();
    });

    // Go to the next step of the form
    $(&quot; , &quot;'&quot; , &quot;#frb-continue&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        e.preventDefault();
        var status = {amount: false, method: false};

        // Validate amount
        if( frb.validateAmount() ){
            status.amount = true;
        } else {
            frb.extraData.validateError = 1;
        }

        // Validate method
        if ($(&quot; , &quot;'&quot; , &quot;input[name=&quot;frb-methods&quot;]:checked&quot; , &quot;'&quot; , &quot;).length === 1) {
            status.method = true;
        } else {
            frb.extraData.validateError = 1;
            $(&quot; , &quot;'&quot; , &quot;.frb-methods&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-haserror&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-error-method&quot; , &quot;'&quot; , &quot;).show();
        }

        if (status.amount === true &amp;&amp; status.method === true) {

            frb.updateUpsellAsk();

            $(&quot; , &quot;'&quot; , &quot;.frb-rml-link, .frb-rml&quot; , &quot;'&quot; , &quot;).hide();

            if ( frb.optinRequired ) {
                frb.showStep(&quot; , &quot;'&quot; , &quot;optin&quot; , &quot;'&quot; , &quot;);
                setTimeout( () => {
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).css({ &quot; , &quot;'&quot; , &quot;visibility&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot; });
                }, 300 );
            } else if ( frb.shouldShowMonthlyConvert() ) {
                frb.showStep(&quot; , &quot;'&quot; , &quot;upsell&quot; , &quot;'&quot; , &quot;);
                setTimeout( () => {
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).css({ &quot; , &quot;'&quot; , &quot;visibility&quot; , &quot;'&quot; , &quot;: &quot; , &quot;'&quot; , &quot;hidden&quot; , &quot;'&quot; , &quot; });
                }, 300 );
            } else {
                frb.submitForm( frb.storedOptions );
            }
        }
    });

    /* -- Back buttons -- */
    $(&quot; , &quot;'&quot; , &quot;.frb-step-optin .frb-back&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        frb.showStep(&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;);
        return false;
    });

    $(&quot; , &quot;'&quot; , &quot;.frb-step-upsell .frb-back&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        if ( frb.optinRequired ) {
            frb.showStep(&quot; , &quot;'&quot; , &quot;optin&quot; , &quot;'&quot; , &quot;);
        } else {
            frb.showStep(&quot; , &quot;'&quot; , &quot;1&quot; , &quot;'&quot; , &quot;);
        }
        return false;
    });

    $(&quot; , &quot;'&quot; , &quot;.frb-step-monthly-diff-amt .frb-back&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        form.otherMonthlyAmount.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        frb.updateUpsellAsk();
        validAmount = 1;
        frb.activateCTA();
        frb.toggleMonthly(false);
        frb.showStep(&quot; , &quot;'&quot; , &quot;upsell&quot; , &quot;'&quot; , &quot;);
        return false;
    });

    // Donate monthly other amount
    $(&quot; , &quot;'&quot; , &quot;.frb-monthly-diff-amt-link&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        form.otherMonthlyAmount.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
        validAmount = 0;
        frb.activateCTA();
        frb.toggleMonthly(true);
        frb.showStep(&quot; , &quot;'&quot; , &quot;monthly-diff-amt&quot; , &quot;'&quot; , &quot;);
        return false;
    });

    // Validate monthly other amount
    $(&quot; , &quot;'&quot; , &quot;#frb-amt-monthly-other-input&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;input change&quot; , &quot;'&quot; , &quot;, function(e) {
        if ( frb.validateMonthlyAmount() ) {
            validAmount = 1;
            frb.updateUpsellAsk(true);
        } else {
            validAmount = 0;
        }
        frb.activateCTA();
    });

    frb.getMonthlyAmount = function() {
        var amount = null;

        // Check the &quot;monthly other&quot; amount box
        if (form.otherMonthlyAmount.value !== &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
            var otherMonthlyAmount = form.otherMonthlyAmount.value;
            otherMonthlyAmount = otherMonthlyAmount.replace(/[,.](\d)$/, &quot; , &quot;'&quot; , &quot;:$10&quot; , &quot;'&quot; , &quot;);
            otherMonthlyAmount = otherMonthlyAmount.replace(/[,.](\d)(\d)$/, &quot; , &quot;'&quot; , &quot;:$1$2&quot; , &quot;'&quot; , &quot;);
            otherMonthlyAmount = otherMonthlyAmount.replace(/[$£€¥,.]/g, &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            otherMonthlyAmount = otherMonthlyAmount.replace(/:/, &quot; , &quot;'&quot; , &quot;.&quot; , &quot;'&quot; , &quot;);
            amount = otherMonthlyAmount;
        }

        amount = parseFloat(amount);

        if ( isNaN(amount) ) {
            return 0;
        } else {
            var totalMonthlyAmountFormatted = frb.formatCurrency(currency, amount, language);
            $(&quot; , &quot;'&quot; , &quot;.frb-monthly-total&quot; , &quot;'&quot; , &quot;).text(totalMonthlyAmountFormatted);

            return amount;
        }
    };

    frb.validateMonthlyAmount = function() {
        var amount = frb.getMonthlyAmount();
        var currency  = frb.getCurrency( mw.centralNotice.data.country );
        var minAmount = frb.amounts.minimums[ currency ];

        if ( amount === null || isNaN(amount) || amount &lt;= 0 || amount &lt; minAmount ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-error-bigamount&quot; , &quot;'&quot; , &quot;).hide();
            $(&quot; , &quot;'&quot; , &quot;.frb-error-smallamount&quot; , &quot;'&quot; , &quot;).show();
            return false;
        } else if ( amount > frb.maxUSD * minAmount ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-error-bigamount&quot; , &quot;'&quot; , &quot;).show();
            return false;
        } else {
            $(&quot; , &quot;'&quot; , &quot;.frb-error-smallamount, .frb-error-bigamount&quot; , &quot;'&quot; , &quot;).hide();
            return true;
        }
    };

    frb.submitMonthly = function() {
        frb.extraData.monthlyUpsell = 1;
        frb.extraData.originalAmt = frb.getAmount().toString();

        frb.toggleMonthly(true);
        document.getElementById(&quot; , &quot;'&quot; , &quot;input_amount_other&quot; , &quot;'&quot; , &quot;).checked = true;
        document.getElementById(&quot; , &quot;'&quot; , &quot;frb-ptf-checkbox&quot; , &quot;'&quot; , &quot;).checked = false;
        form.otherAmount.value = form.otherMonthlyAmount.value;
        frb.submitForm(frb.storedOptions);
    };

    // Submit form
    $(&quot; , &quot;'&quot; , &quot;#frb-monthly-donate-yes&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        frb.submitMonthly();
        return false;
    });

    $(&quot; , &quot;'&quot; , &quot;#frb-monthly-donate-no&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        frb.submitForm(frb.storedOptions);
        return false;
    });

    $(&quot; , &quot;'&quot; , &quot;#frb-donate-monthly-other&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function (e) {
        if (frb.validateMonthlyAmount()) {
            frb.submitMonthly();
        }
        return false;
    });

    /**
     * Should we show pre-payment monthly convert?
     *
     * Only if: initial selection is one-time, suggested amount is not 0 (meaning skip),
     * payment method supports monthly, and payment method does not have post-payments monthly convert
     *
     * @returns boolean
     */
     frb.shouldShowMonthlyConvert = function() {
        let postPaymentMethods = [ &quot; , &quot;'&quot; , &quot;cc&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;apple&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;google&quot; , &quot;'&quot; , &quot; ];
        if (
            frb.getRecurring( document.getElementById( &quot; , &quot;'&quot; , &quot;frb-form&quot; , &quot;'&quot; , &quot; ) ) ||
            !frb.shouldShowRecurring( frb.storedOptions, mw.centralNotice.data.country ) ||
            form.otherMonthlyAmount.value == 0 ||
            postPaymentMethods.includes( frb.storedOptions.method )
        ) {
            return false;
        } else {
            return true;
        }
    }

    $(&quot; , &quot;'&quot; , &quot;#frb-donate&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        if ( frb.validateForm( frb.storedOptions) ) {
            if ( frb.shouldShowMonthlyConvert() ) {
                frb.showStep(&quot; , &quot;'&quot; , &quot;upsell&quot; , &quot;'&quot; , &quot;);
            } else {
                frb.submitForm( frb.storedOptions );
            }
        } else {
            frb.extraData.validateError = 1;
        }
        return false;
    });

    // Focus for #input_amount_other
    $(&quot; , &quot;'&quot; , &quot;.frb-amt-other&quot; , &quot;'&quot; , &quot;).click(function() {
        document.getElementById(&quot; , &quot;'&quot; , &quot;input_amount_other&quot; , &quot;'&quot; , &quot;).checked = true;
        frb.updateFeeDisplay();
        $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).focus();
    });

    // Activate #input_amount_other radio when tabbing into #frb-amt-other-input
    $(&quot; , &quot;'&quot; , &quot;#frb-amt-other-input&quot; , &quot;'&quot; , &quot;).focus(function() {
        document.getElementById(&quot; , &quot;'&quot; , &quot;input_amount_other&quot; , &quot;'&quot; , &quot;).checked = true;
        frb.updateFeeDisplay();
    });

    frb.activateCTA = function(){
        if ( validAmount &amp;&amp; validMethod ) {
            $(&quot; , &quot;'&quot; , &quot;.frb-submit-txt-continue&quot; , &quot;'&quot; , &quot;).hide();
            $(&quot; , &quot;'&quot; , &quot;.frb-submit-txt-donate&quot; , &quot;'&quot; , &quot;).show();
            $(&quot; , &quot;'&quot; , &quot;#frb-continue, #frb-monthly-donate-yes, #frb-monthly-donate-no, #frb-donate-monthly-other&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
            if (validOptin) {
                $(&quot; , &quot;'&quot; , &quot;#frb-donate&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
            }
        } else {
            $(&quot; , &quot;'&quot; , &quot;.frb-submit&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        }
    };


    /* --- Nag/minimized banner functionality --- */

    // On Load
    var bannerOuterHeight = $(&quot; , &quot;'&quot; , &quot;.frb-in-article&quot; , &quot;'&quot; , &quot;).outerHeight( true );
    var stickyHeaderTop = bannerOuterHeight + $(&quot; , &quot;'&quot; , &quot;.frb-in-article&quot; , &quot;'&quot; , &quot;).offset().top + 200;

    frb.initNag = function() {

        // Intercept TOC clicks, and account for nag height
        $(&quot; , &quot;'&quot; , &quot;#toc ul > li a&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click.frb&quot; , &quot;'&quot; , &quot;, function(e) {
            e.preventDefault();

            var anchor = $(this).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;).replace(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            anchor = $(&quot;[id=&quot; , &quot;'&quot; , &quot;&quot;+$.escapeSelector(anchor)+&quot;&quot; , &quot;'&quot; , &quot;]&quot;);

            var offsetTop = anchor.offset().top - $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).outerHeight();
            $(&quot; , &quot;'&quot; , &quot;body, html&quot; , &quot;'&quot; , &quot;).animate({ scrollTop: offsetTop }, 10);
            window.location.hash = $(this).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;);
        });

        // Scroll to section, accounting for nag height
        if ( window.location.hash ) {
            var offsetTop;
            var hash = decodeURI(window.location.hash).replace(&quot; , &quot;'&quot; , &quot;#&quot; , &quot;'&quot; , &quot;,&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
            hash = $(&quot;[id=&quot; , &quot;'&quot; , &quot;&quot;+$.escapeSelector(hash)+&quot;&quot; , &quot;'&quot; , &quot;]&quot;);
            if ( hash.offset() ) { // T281547
                offsetTop = hash.offset().top + bannerOuterHeight - $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).outerHeight();
                $(&quot; , &quot;'&quot; , &quot;body, html&quot; , &quot;'&quot; , &quot;).animate( { scrollTop: offsetTop }, 100 );
            }
        }

        $(window).on(&quot; , &quot;'&quot; , &quot;resize.frb&quot; , &quot;'&quot; , &quot;, function() {
            bannerOuterHeight = $(&quot; , &quot;'&quot; , &quot;.frb-in-article&quot; , &quot;'&quot; , &quot;).outerHeight( true );
            stickyHeaderTop = bannerOuterHeight + $(&quot; , &quot;'&quot; , &quot;.frb-in-article&quot; , &quot;'&quot; , &quot;).offset().top + 200;
        });

        function scrollFunction() {
            if( $(window).scrollTop() > stickyHeaderTop ) {
                if ( !frb.fixed ) {
                    $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).show();
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml-form-wrapper&quot; , &quot;'&quot; , &quot;).hide();
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;.frb-nag .frb-form-wrapper&quot; , &quot;'&quot; , &quot;);
                }
            } else {
                if ( frb.fixed ) {
                    $(&quot; , &quot;'&quot; , &quot;.frb-prevent-page-jump&quot; , &quot;'&quot; , &quot;)
                        .removeClass(&quot; , &quot;'&quot; , &quot;frb-in-article&quot; , &quot;'&quot; , &quot;)
                        .hide();
                    $(&quot; , &quot;'&quot; , &quot;#frb-main&quot; , &quot;'&quot; , &quot;)
                        .removeClass(&quot; , &quot;'&quot; , &quot;frb-fixed&quot; , &quot;'&quot; , &quot;)
                        .addClass(&quot; , &quot;'&quot; , &quot;frb-in-article&quot; , &quot;'&quot; , &quot;)
                        .trigger(&quot; , &quot;'&quot; , &quot;unFixed&quot; , &quot;'&quot; , &quot;);
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;#frb-main .frb-form-wrapper&quot; , &quot;'&quot; , &quot;);
                    frb.fixed = false;
                } else {
                    $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).hide();
                    $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;#frb-main .frb-form-wrapper&quot; , &quot;'&quot; , &quot;);
                }
            }
        }

        $(window).on(&quot; , &quot;'&quot; , &quot;load.frb scroll.frb resize.frb&quot; , &quot;'&quot; , &quot;, function() {
            scrollFunction();
        });

        frb.clickNag = function(e) {
            frb.extraData.clickedNag = 1;

            if ( window.innerHeight &lt; document.getElementById(&quot; , &quot;'&quot; , &quot;frb-main&quot; , &quot;'&quot; , &quot;).offsetHeight ) {
                // Window height too short for fixing position, just jump to main banner
                window.scrollTo(0, 0);
                return false;
            }

            // Add spacer to prevent jump
            var inArticleHeight = $(&quot; , &quot;'&quot; , &quot;#frb-main&quot; , &quot;'&quot; , &quot;).outerHeight();
            $(&quot; , &quot;'&quot; , &quot;.frb-prevent-page-jump&quot; , &quot;'&quot; , &quot;)
                .height( inArticleHeight )
                .addClass(&quot; , &quot;'&quot; , &quot;frb-in-article&quot; , &quot;'&quot; , &quot;) // So that it can be used for stickyHeader calcs
                .show();

            $(&quot; , &quot;'&quot; , &quot;#frb-main&quot; , &quot;'&quot; , &quot;)
                .removeClass(&quot; , &quot;'&quot; , &quot;frb-in-article&quot; , &quot;'&quot; , &quot;)
                .addClass(&quot; , &quot;'&quot; , &quot;frb-fixed&quot; , &quot;'&quot; , &quot;);

            $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).appendTo(&quot; , &quot;'&quot; , &quot;#frb-main .frb-form-wrapper&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).hide();

            frb.fixed = true;
            return false;
        };

        $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).on( &quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, frb.clickNag );
        $(&quot; , &quot;'&quot; , &quot;#nag-yes-btn&quot; , &quot;'&quot; , &quot;).on( &quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, frb.clickNag );

        $(&quot; , &quot;'&quot; , &quot;#nag-rml-btn&quot; , &quot;'&quot; , &quot;).on( &quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
            // Add &quot; , &quot;'&quot; , &quot;_nag&quot; , &quot;'&quot; , &quot; to RML source value
            $(&quot; , &quot;'&quot; , &quot;#frb-rml-form input[name=&quot;rml_source&quot;]&quot; , &quot;'&quot; , &quot;).val(&quot; , &quot;'&quot; , &quot;B2324_121810_en6C_dsk_p1_lg_txt_169C_nag&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-rml-displayed&quot; , &quot;'&quot; , &quot;);
            $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).off(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;); // Remove so they can interact with RML
            $(&quot; , &quot;'&quot; , &quot;#frb-rml-email&quot; , &quot;'&quot; , &quot;).focus();
            $(&quot; , &quot;'&quot; , &quot;.frb-rml-close-wrapper&quot; , &quot;'&quot; , &quot;).hide();
        });

    };

    $(&quot; , &quot;'&quot; , &quot;.back-rml&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        $(&quot; , &quot;'&quot; , &quot;.frb-nag&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;frb-rml-displayed&quot; , &quot;'&quot; , &quot;);
    });

    // Close inline rml form on click or return
    $(&quot; , &quot;'&quot; , &quot;.frb-rml-close&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function (e) {
        $(&quot; , &quot;'&quot; , &quot;.frb-rml-form-wrapper&quot; , &quot;'&quot; , &quot;).hide();
        e.stopPropagation();
    });

    $(&quot; , &quot;'&quot; , &quot;.frb-close&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function (e) {
        frb.hide();
        frb.altSetHideCookie( &quot; , &quot;'&quot; , &quot;close&quot; , &quot;'&quot; , &quot;, frb.HIDE_DURATION_CLOSE );
        frb.showSidebarTooltip();
        return false;
    });

    // Open already donated modal
    $(&quot; , &quot;'&quot; , &quot;.modal-open&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        $(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).show();
        $(&quot; , &quot;'&quot; , &quot;button.modal-close-x&quot; , &quot;'&quot; , &quot;).focus();
    });

    // Close already donated modal
    $(&quot; , &quot;'&quot; , &quot;.modal-close&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;click&quot; , &quot;'&quot; , &quot;, function(e) {
        closeModal(e);
        $(&quot; , &quot;'&quot; , &quot;.modal-open&quot; , &quot;'&quot; , &quot;).focus();
    });

    $(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).click(function(e) {
        const modalContainer = $(&quot; , &quot;'&quot; , &quot;.modal-container&quot; , &quot;'&quot; , &quot;);

        if (!modalContainer.is(e.target) &amp;&amp; modalContainer.has(e.target).length === 0) {
            closeModal(e);
            $(&quot; , &quot;'&quot; , &quot;.modal-open&quot; , &quot;'&quot; , &quot;).focus();
        }
    });

    function closeModal(e) {
        $(&quot; , &quot;'&quot; , &quot;.modal&quot; , &quot;'&quot; , &quot;).hide();
        if (e.target.name == &quot; , &quot;'&quot; , &quot;frb-modal-close-button&quot; , &quot;'&quot; , &quot;) {
            frb.hide();
            frb.altSetHideCookie( &quot; , &quot;'&quot; , &quot;donate close&quot; , &quot;'&quot; , &quot;, 604800 );
            e.target.blur();
        }
        return false;
    };

    $(&quot; , &quot;'&quot; , &quot;#frb-main&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;unFixed&quot; , &quot;'&quot; , &quot;, function() {
        $(&quot; , &quot;'&quot; , &quot;.frb-rml&quot; , &quot;'&quot; , &quot;).css( &quot; , &quot;'&quot; , &quot;top&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;.frb-rml-link&quot; , &quot;'&quot; , &quot;).position().top + 40 );
    });

    if ( country == &quot;US&quot; || country == &quot;CA&quot; || country == &quot;GB&quot; || country == &quot;IE&quot; || country == &quot;AU&quot; || country == &quot;NZ&quot; ) {
        $(&quot; , &quot;'&quot; , &quot;#frb-main&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;frb-6c&quot; , &quot;'&quot; , &quot;);
    }

    if ( frb.shouldShowBanner() ) {
        frb.initNag();
        frb.show();
    }

});

		&quot;))]</value>
      <webElementGuid>c1a30d82-b559-4db4-9c6a-ab74397c2a4f</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
