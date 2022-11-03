macro_rules! builder_constructors {
    (
        $(
            $(#[$attr:meta])*
            $name:ident {
                $(
                    $(#[$attr_method:meta])*
                    $fil:ident$(: $vil:literal)?,
                )*
            }
         )*
    ) => {
        /// All built-in attributes
        #[allow(non_cammel_case)]
        pub enum Attribute {
            $(
                $(
                    $(#[$attr_method])*
                    $fil,
                )*
            )*
        }
    };
}

builder_constructors! {
    a {
        // Indicates that the hyperlink is to be used for downloading a resource.
        download,
        // The URL of a linked resource.
        href,
        // Specifies the language of the linked resource.
        hreflang,
        // Specifies a hint of the media for which the linked resource was designed.
        media,
        // The ping attribute specifies a space-separated list of URLs to be notified if a user follows the hyperlink.
        ping,
        // Specifies which referrer is sent when fetching the resource.
        referrerpolicy,
        // Specifies the relationship of the target object to the link object.
        rel,
        //
        shape,
        // Specifies where to open the linked document (in the case of an <a> element) or where to display the response received (in the case of a <form> element)
        target,
    }
    applet {
        // Specifies the horizontal alignment of the element.
        align,
        // Alternative text in case an image can't be displayed.
        alt,
        // Specifies the URL of the applet's class file to be loaded and executed.
        code,
        // This attribute gives the absolute or relative URL of the directory where applets' .class files referenced by the code attribute are stored.
        codebase,
    }
    area {
        // Alternative text in case an image can't be displayed.
        alt,
        // A set of values specifying the coordinates of the hot-spot region.
        coords,
        // Indicates that the hyperlink is to be used for downloading a resource.
        download,
        // The URL of a linked resource.
        href,
        // Specifies the language of the linked resource.
        hreflang,
        // Specifies a hint of the media for which the linked resource was designed.
        media,
        // The ping attribute specifies a space-separated list of URLs to be notified if a user follows the hyperlink.
        ping,
        // Specifies which referrer is sent when fetching the resource.
        referrerpolicy,
        // Specifies the relationship of the target object to the link object.
        rel,
        //
        shape,
        // Specifies where to open the linked document (in the case of an <a> element) or where to display the response received (in the case of a <form> element)
        target,
    }
    audio {
        // The audio or video should play as soon as possible.
        autoplay,
        // Contains the time range of already buffered media.
        buffered,
        // Indicates whether the browser should show playback controls to the user.
        controls,
        // How the element handles cross-origin requests
        crossorigin,
        // Indicates whether the media should start playing from the start when it's finished.
        r#loop,
        // Indicates whether the audio will be initially silenced on page load.
        muted,
        // Indicates whether the whole resource, parts of it or nothing should be preloaded.
        preload,
        // The URL of the embeddable content.
        src,
    }
    base {
        // The URL of a linked resource.
        href,
        // Specifies where to open the linked document (in the case of an <a> element) or where to display the response received (in the case of a <form> element)
        target,
    }
    bgsound {
        // Indicates whether the media should start playing from the start when it's finished.
        r#loop,
    }
    blockquote {
        // Contains a URI which points to the source of the quote or change.
        cite,
    }
    body {
        // Specifies the URL of an image file.
        background,
        // Background color of the element.
        bgcolor,
    }
    button {
        // The element should be automatically focused after the page loaded.
        autofocus,
        // Indicates whether the user can interact with the element.
        disabled,
        // Indicates the form that is the owner of the element.
        form,
        // Indicates the action of the element, overriding the action defined in the <form>.
        formaction,
        // If the button/input is a submit button (type="submit"), this attribute sets the encoding type to use during form submission. If this attribute is specified, it overrides the enctype attribute of the button's form owner.
        formenctype,
        // If the button/input is a submit button (type="submit"), this attribute sets the submission method to use during form submission (GET, POST, etc.). If this attribute is specified, it overrides the method attribute of the button's form owner.
        formmethod,
        // If the button/input is a submit button (type="submit"), this boolean attribute specifies that the form is not to be validated when it is submitted. If this attribute is specified, it overrides the novalidate attribute of the button's form owner.
        formnovalidate,
        // If the button/input is a submit button (type="submit"), this attribute specifies the browsing context (for example, tab, window, or inline frame) in which to display the response that is received after submitting the form. If this attribute is specified, it overrides the target attribute of the button's form owner.
        formtarget,
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
        // Defines the type of the element.
        r#type,
        // Defines a default value which will be displayed in the element on page load.
        value,
    }
    canvas {
        // Specifies the height of elements listed here. For all other elements, use the CSS height property.
        height,
        // For the elements listed here, this establishes the element's width.
        width,
    }
    caption {
        // Specifies the horizontal alignment of the element.
        align,
    }
    col {
        // Specifies the horizontal alignment of the element.
        align,
        // Background color of the element.
        bgcolor,
        //
        span,
    }
    colgroup {
        // Specifies the horizontal alignment of the element.
        align,
        // Background color of the element.
        bgcolor,
        //
        span,
    }
    command {
        // Indicates whether the element should be checked on page load.
        checked,
        // Indicates whether the user can interact with the element.
        disabled,
        // Specifies a picture which represents the command.
        icon,
        //
        radiogroup,
        // Defines the type of the element.
        r#type,
    }
    contenteditable {
        // The enterkeyhint specifies what action label (or icon) to present for the enter key on virtual keyboards. The attribute can be used with form controls (such as the value of textarea elements), or in elements in an editing host (e.g., using contenteditable attribute).
        enterkeyhint,
        // Provides a hint as to the type of data that might be entered by the user while editing the element or its contents. The attribute can be used with form controls (such as the value of textarea elements), or in elements in an editing host (e.g., using contenteditable attribute).
        inputmode,
    }
    data {
        // Defines a default value which will be displayed in the element on page load.
        value,
    }
    del {
        // Contains a URI which points to the source of the quote or change.
        cite,
        // Indicates the date and time associated with the element.
        datetime,
    }
    details {
        // Indicates whether the contents are currently visible (in the case of a <details> element) or whether the dialog is active and can be interacted with (in the case of a <dialog> element).
        open,
    }
    dialog {
        // Indicates whether the contents are currently visible (in the case of a <details> element) or whether the dialog is active and can be interacted with (in the case of a <dialog> element).
        open,
    }
    embed {
        // Specifies the height of elements listed here. For all other elements, use the CSS height property.
        height,
        // The URL of the embeddable content.
        src,
        // Defines the type of the element.
        r#type,
        // For the elements listed here, this establishes the element's width.
        width,
    }
    fieldset {
        // Indicates whether the user can interact with the element.
        disabled,
        // Indicates the form that is the owner of the element.
        form,
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
    }
    font {
        // This attribute sets the text color using either a named color or a color specified in the hexadecimal #RRGGBB format.
        color,
    }
    form {
        // List of types the server accepts, typically a file type.
        accept,
        // List of supported charsets.
        accept_charset: "accept-charset",
        // The URI of a program that processes the information submitted via the form.
        action,
        // Indicates whether controls in this form can by default have their values automatically completed by the browser.
        autocomplete,
        // Defines the content type of the form data when the method is POST.
        enctype,
        // Defines which HTTP method to use when submitting the form. Can be GET (default) or POST.
        method,
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
        // This attribute indicates that the form shouldn't be validated when submitted.
        novalidate,
        // Specifies where to open the linked document (in the case of an <a> element) or where to display the response received (in the case of a <form> element)
        target,
    }
    hr {
        // Specifies the horizontal alignment of the element.
        align,
        // This attribute sets the text color using either a named color or a color specified in the hexadecimal #RRGGBB format.
        color,
    }
    html {
        // Specifies the URL of the document's cache manifest.
        manifest,
    }
    iframe {
        // Specifies the horizontal alignment of the element.
        align,
        // Specifies a feature-policy for the iframe.
        allow,
        // Specifies the Content Security Policy that an embedded document must agree to enforce upon itself.
        csp,
        // Specifies the height of elements listed here. For all other elements, use the CSS height property.
        height,
        // Indicates the relative fetch priority for the resource.
        importance,
        // Indicates if the element should be loaded lazily (loading="lazy") or loaded immediately (loading="eager").
        loading,
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
        // Specifies which referrer is sent when fetching the resource.
        referrerpolicy,
        // Stops a document loaded in an iframe from using certain features (such as submitting forms or opening new windows).
        sandbox,
        // The URL of the embeddable content.
        src,
        //
        srcdoc,
        // For the elements listed here, this establishes the element's width.
        width,
    }
    img {
        // Specifies the horizontal alignment of the element.
        align,
        // Alternative text in case an image can't be displayed.
        alt,
        // The border width.
        border,
        // How the element handles cross-origin requests
        crossorigin,
        // Indicates the preferred method to decode the image.
        decoding,
        // Specifies the height of elements listed here. For all other elements, use the CSS height property.
        height,
        // Indicates the relative fetch priority for the resource.
        importance,
        // This attribute tells the browser to ignore the actual intrinsic size of the image and pretend it's the size specified in the attribute.
        intrinsicsize,
        // Indicates that the image is part of a server-side image map.
        ismap,
        // Indicates if the element should be loaded lazily (loading="lazy") or loaded immediately (loading="eager").
        loading,
        // Specifies which referrer is sent when fetching the resource.
        referrerpolicy,
        //
        sizes,
        // The URL of the embeddable content.
        src,
        // One or more responsive image candidates.
        srcset,
        //
        usemap,
        // For the elements listed here, this establishes the element's width.
        width,
    }
    input {
        // List of types the server accepts, typically a file type.
        accept,
        // Alternative text in case an image can't be displayed.
        alt,
        // Indicates whether controls in this form can by default have their values automatically completed by the browser.
        autocomplete,
        // The element should be automatically focused after the page loaded.
        autofocus,
        // From the Media Capture specification, specifies a new file can be captured.
        capture,
        // Indicates whether the element should be checked on page load.
        checked,
        //
        dirname,
        // Indicates whether the user can interact with the element.
        disabled,
        // Indicates the form that is the owner of the element.
        form,
        // Indicates the action of the element, overriding the action defined in the <form>.
        formaction,
        // If the button/input is a submit button (type="submit"), this attribute sets the encoding type to use during form submission. If this attribute is specified, it overrides the enctype attribute of the button's form owner.
        formenctype,
        // If the button/input is a submit button (type="submit"), this attribute sets the submission method to use during form submission (GET, POST, etc.). If this attribute is specified, it overrides the method attribute of the button's form owner.
        formmethod,
        // If the button/input is a submit button (type="submit"), this boolean attribute specifies that the form is not to be validated when it is submitted. If this attribute is specified, it overrides the novalidate attribute of the button's form owner.
        formnovalidate,
        // If the button/input is a submit button (type="submit"), this attribute specifies the browsing context (for example, tab, window, or inline frame) in which to display the response that is received after submitting the form. If this attribute is specified, it overrides the target attribute of the button's form owner.
        formtarget,
        // Specifies the height of elements listed here. For all other elements, use the CSS height property.
        height,
        // Identifies a list of pre-defined options to suggest to the user.
        list,
        // Indicates the maximum value allowed.
        max,
        // Defines the maximum number of characters allowed in the element.
        maxlength,
        // Indicates the minimum value allowed.
        min,
        // Defines the minimum number of characters allowed in the element.
        minlength,
        // Indicates whether multiple values can be entered in an input of the type email or file.
        multiple,
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
        // Defines a regular expression which the element's value will be validated against.
        pattern,
        // Provides a hint to the user of what can be entered in the field.
        placeholder,
        // Indicates whether the element can be edited.
        readonly,
        // Indicates whether this element is required to fill out or not.
        required,
        // Defines the width of the element (in pixels). If the element's type attribute is text or password then it's the number of characters.
        size,
        // The URL of the embeddable content.
        src,
        //
        step,
        // Defines the type of the element.
        r#type,
        //
        usemap,
        // Defines a default value which will be displayed in the element on page load.
        value,
        // For the elements listed here, this establishes the element's width.
        width,
    }
    ins {
        // Contains a URI which points to the source of the quote or change.
        cite,
        // Indicates the date and time associated with the element.
        datetime,
    }
    keygen {
        // The element should be automatically focused after the page loaded.
        autofocus,
        // A challenge string that is submitted along with the public key.
        challenge,
        // Indicates whether the user can interact with the element.
        disabled,
        // Indicates the form that is the owner of the element.
        form,
        // Specifies the type of key generated.
        keytype,
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
    }
    label {
        // Describes elements which belongs to this one.
        r#for,
        // Indicates the form that is the owner of the element.
        form,
    }
    li {
        // Defines a default value which will be displayed in the element on page load.
        value,
    }
    link {
        // How the element handles cross-origin requests
        crossorigin,
        // The URL of a linked resource.
        href,
        // Specifies the language of the linked resource.
        hreflang,
        // Indicates the relative fetch priority for the resource.
        importance,
        // Specifies a Subresource Integrity value that allows browsers to verify what they fetch.
        integrity,
        // Specifies a hint of the media for which the linked resource was designed.
        media,
        // Specifies which referrer is sent when fetching the resource.
        referrerpolicy,
        // Specifies the relationship of the target object to the link object.
        rel,
        //
        sizes,
        // Defines the type of the element.
        r#type,
    }
    map {
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
    }
    marquee {
        // Background color of the element.
        bgcolor,
        // Indicates whether the media should start playing from the start when it's finished.
        r#loop,
    }
    menu {
        // Defines the type of the element.
        r#type,
    }
    meta {
        // Declares the character encoding of the page or script.
        charset,
        // A value associated with http-equiv or name depending on the context.
        content,
        // Defines a pragma directive.
        http_equiv: "http-equiv",
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
    }
    meter {
        // Indicates the form that is the owner of the element.
        form,
        // Indicates the lower bound of the upper range.
        high,
        // Indicates the upper bound of the lower range.
        low,
        // Indicates the maximum value allowed.
        max,
        // Indicates the minimum value allowed.
        min,
        // Indicates the optimal numeric value.
        optimum,
        // Defines a default value which will be displayed in the element on page load.
        value,
    }
    object {
        // The border width.
        border,
        // Specifies the URL of the resource.
        data,
        // Indicates the form that is the owner of the element.
        form,
        // Specifies the height of elements listed here. For all other elements, use the CSS height property.
        height,
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
        // Defines the type of the element.
        r#type,
        //
        usemap,
        // For the elements listed here, this establishes the element's width.
        width,
    }
    ol {
        // Indicates whether the list should be displayed in a descending order instead of a ascending.
        reversed,
        // Defines the first number if other than 1.
        start,
    }
    optgroup {
        // Indicates whether the user can interact with the element.
        disabled,
        // Specifies a user-readable title of the element.
        label,
    }
    option {
        // Indicates whether the user can interact with the element.
        disabled,
        // Specifies a user-readable title of the element.
        label,
        // Defines a value which will be selected on page load.
        selected,
        // Defines a default value which will be displayed in the element on page load.
        value,
    }
    output {
        // Describes elements which belongs to this one.
        r#for,
        // Indicates the form that is the owner of the element.
        form,
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
    }
    param {
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
        // Defines a default value which will be displayed in the element on page load.
        value,
    }
    progress {
        // Indicates the form that is the owner of the element.
        form,
        // Indicates the maximum value allowed.
        max,
        // Defines a default value which will be displayed in the element on page load.
        value,
    }
    q {
        // Contains a URI which points to the source of the quote or change.
        cite,
    }
    script {
        // Executes the script asynchronously.
        r#async,
        // Declares the character encoding of the page or script.
        charset,
        // How the element handles cross-origin requests
        crossorigin,
        // Indicates that the script should be executed after the page has been parsed.
        defer,
        // Indicates the relative fetch priority for the resource.
        importance,
        // Specifies a Subresource Integrity value that allows browsers to verify what they fetch.
        integrity,
        // Defines the script language used in the element.
        language,
        // Specifies which referrer is sent when fetching the resource.
        referrerpolicy,
        // The URL of the embeddable content.
        src,
        // Defines the type of the element.
        r#type,
    }
    select {
        // Indicates whether controls in this form can by default have their values automatically completed by the browser.
        autocomplete,
        // The element should be automatically focused after the page loaded.
        autofocus,
        // Indicates whether the user can interact with the element.
        disabled,
        // Indicates the form that is the owner of the element.
        form,
        // Indicates whether multiple values can be entered in an input of the type email or file.
        multiple,
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
        // Indicates whether this element is required to fill out or not.
        required,
        // Defines the width of the element (in pixels). If the element's type attribute is text or password then it's the number of characters.
        size,
    }
    source {
        // Specifies a hint of the media for which the linked resource was designed.
        media,
        //
        sizes,
        // The URL of the embeddable content.
        src,
        // One or more responsive image candidates.
        srcset,
        // Defines the type of the element.
        r#type,
    }
    style {
        // Specifies a hint of the media for which the linked resource was designed.
        media,
        //
        scoped,
        // Defines the type of the element.
        r#type,
    }
    table {
        // Specifies the horizontal alignment of the element.
        align,
        // Specifies the URL of an image file.
        background,
        // Background color of the element.
        bgcolor,
        // The border width.
        border,
        //
        summary,
    }
    tbody {
        // Specifies the horizontal alignment of the element.
        align,
        // Background color of the element.
        bgcolor,
    }
    td {
        // Specifies the horizontal alignment of the element.
        align,
        // Specifies the URL of an image file.
        background,
        // Background color of the element.
        bgcolor,
        // The colspan attribute defines the number of columns a cell should span.
        colspan,
        // IDs of the <th> elements which applies to this element.
        headers,
        // Defines the number of rows a table cell should span over.
        rowspan,
    }
    textarea {
        // Indicates whether controls in this form can by default have their values automatically completed by the browser.
        autocomplete,
        // The element should be automatically focused after the page loaded.
        autofocus,
        // Defines the number of columns in a textarea.
        cols,
        //
        dirname,
        // Indicates whether the user can interact with the element.
        disabled,
        // The enterkeyhint specifies what action label (or icon) to present for the enter key on virtual keyboards. The attribute can be used with form controls (such as the value of textarea elements), or in elements in an editing host (e.g., using contenteditable attribute).
        enterkeyhint,
        // Indicates the form that is the owner of the element.
        form,
        // Provides a hint as to the type of data that might be entered by the user while editing the element or its contents. The attribute can be used with form controls (such as the value of textarea elements), or in elements in an editing host (e.g., using contenteditable attribute).
        inputmode,
        // Defines the maximum number of characters allowed in the element.
        maxlength,
        // Defines the minimum number of characters allowed in the element.
        minlength,
        // Name of the element. For example used by the server to identify the fields in form submits.
        name,
        // Provides a hint to the user of what can be entered in the field.
        placeholder,
        // Indicates whether the element can be edited.
        readonly,
        // Indicates whether this element is required to fill out or not.
        required,
        // Defines the number of rows in a text area.
        rows,
        // Indicates whether the text should be wrapped.
        wrap,
    }
    tfoot {
        // Specifies the horizontal alignment of the element.
        align,
        // Background color of the element.
        bgcolor,
    }
    th {
        // Specifies the horizontal alignment of the element.
        align,
        // Specifies the URL of an image file.
        background,
        // Background color of the element.
        bgcolor,
        // The colspan attribute defines the number of columns a cell should span.
        colspan,
        // IDs of the <th> elements which applies to this element.
        headers,
        // Defines the number of rows a table cell should span over.
        rowspan,
        // Defines the cells that the header test (defined in the th element) relates to.
        scope,
    }
    thead {
        // Specifies the horizontal alignment of the element.
        align,
    }
    time {
        // Indicates the date and time associated with the element.
        datetime,
    }
    tr {
        // Specifies the horizontal alignment of the element.
        align,
        // Background color of the element.
        bgcolor,
    }
    track {
        // Indicates that the track should be enabled unless the user's preferences indicate something different.
        default,
        // Specifies the kind of text track.
        kind,
        // Specifies a user-readable title of the element.
        label,
        // The URL of the embeddable content.
        src,
        //
        srclang,
    }
    video {
        // The audio or video should play as soon as possible.
        autoplay,
        // Contains the time range of already buffered media.
        buffered,
        // Indicates whether the browser should show playback controls to the user.
        controls,
        // How the element handles cross-origin requests
        crossorigin,
        // Specifies the height of elements listed here. For all other elements, use the CSS height property.
        height,
        // Indicates whether the media should start playing from the start when it's finished.
        r#loop,
        // Indicates whether the audio will be initially silenced on page load.
        muted,
        // A URL indicating a poster frame to show until the user plays or seeks.
        poster,
        // Indicates whether the whole resource, parts of it or nothing should be preloaded.
        preload,
        // The URL of the embeddable content.
        src,
        // For the elements listed here, this establishes the element's width.
        width,
    }
}
