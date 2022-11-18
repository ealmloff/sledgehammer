use sledgehammer_encoder::*;
use sledgehammer_prebuild::html;

fn test() {
    const EL: StaticBatch = html! {
        <tr sledgehammer-id="2">
            <td class="col-md-1">
            </td>
            <td class="col-md-4">
                <a class="lbl">
                </a>
            </td>
            <td class="col-md-1">
                <a class="remove">
                    <span class="remove glyphicon glyphicon-remove" aria-hidden="true">
                    </span>
                </a>
            </td>
            <td class="col-md-6">
            </td>
        </tr>
    };
}
