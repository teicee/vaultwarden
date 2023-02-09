"use strict";

function deleteOrganization() {
    event.preventDefault();
    event.stopPropagation();
    const org_uuid = event.target.dataset.vwOrgUuid;
    const org_name = event.target.dataset.vwOrgName;
    const billing_email = event.target.dataset.vwBillingEmail;
    if (!org_uuid) {
        alert("Required parameters not found!");
        return false;
    }

    // First make sure the user wants to delete this organization
    const continueDelete = confirm(`WARNING: All data of this organization (${org_name}) will be lost!\nMake sure you have a backup, this cannot be undone!`);
    if (continueDelete == true) {
        const input_org_uuid = prompt(`To delete the organization "${org_name} (${billing_email})", please type the organization uuid below.`);
        if (input_org_uuid != null) {
            if (input_org_uuid == org_uuid) {
                _post(`${BASE_URL}/admin/organizations/${org_uuid}/delete`,
                    "Organization deleted correctly",
                    "Error deleting organization"
                );
            } else {
                alert("Wrong organization uuid, please try again");
            }
        }
    }
}

function getFormData() {
    let data = {};

    document.querySelectorAll(".conf-checkbox").forEach(function (e) {
        data[e.name] = e.checked;
    });

    document.querySelectorAll(".conf-number").forEach(function (e) {
        data[e.name] = e.value ? +e.value : null;
    });

    document.querySelectorAll(".conf-text, .conf-password").forEach(function (e) {
        data[e.name] = e.value || null;
    });
    return data;
}

// Two functions to help check if there were changes to the form fields
// Useful for example during the smtp test to prevent people from clicking save before testing there new settings
function initChangeDetection(form) {
    const ignore_fields = ["smtp-test-email"];
    Array.from(form).forEach((el) => {
        if (! ignore_fields.includes(el.id)) {
            el.dataset.origValue = el.value;
        }
    });
}

// This function will prevent submitting a from when someone presses enter.
function preventFormSubmitOnEnter(form) {
    form.onkeypress = function(e) {
        const key = e.charCode || e.keyCode || 0;
        if (key == 13) {
            e.preventDefault();
        }
    };
}

function saveSsoConfig() {
    const data = JSON.stringify(getFormData());
    console.log(data);
    _post(`${BASE_URL}/admin/sso_settings/`,
        "Config saved correctly",
        "Error saving config",
        data
    );
    event.preventDefault();
}

const sso_config_form = document.getElementById("sso-config-form");

// onLoad events
document.addEventListener("DOMContentLoaded", (/*event*/) => {
    initChangeDetection(sso_config_form);
    preventFormSubmitOnEnter(sso_config_form);
    jQuery("#orgs-table").DataTable({
        "stateSave": true,
        "responsive": true,
        "lengthMenu": [
            [-1, 5, 10, 25, 50],
            ["All", 5, 10, 25, 50]
        ],
        "pageLength": -1, // Default show all
        "columnDefs": [{
            "targets": 4,
            "searchable": false,
            "orderable": false
        }]
    });

    // Add click events for organization actions
    document.querySelectorAll("button[vw-delete-organization]").forEach(btn => {
        btn.addEventListener("click", deleteOrganization);
    });

    document.getElementById("reload").addEventListener("click", reload);
    sso_config_form.addEventListener("submit", saveSsoConfig);
});