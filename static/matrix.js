console.log($('select'))
$('select').on('change', function (e) {
    var optionSelected = $("option:selected", this);
    var valueSelected = this.value;
    if(valueSelected == "Custom kernel..."){
        $('.input-group').map(function() {
            return $( this ).removeAttr('hidden');
        });
    } else {
        $('.input-group').map(function() {
            return $( this ).attr('hidden', true);
        })
    };
});