<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_AjaxCart.init(false, .header-links .ca_003bf3</name>
   <tag></tag>
   <elementGuidId>85b6cc1c-8842-4a3c-857a-eb6ddfb2ed14</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>2b306970-5498-4d37-9311-ecffe4ad6c44</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    







     





    
    
        
            AjaxCart.init(false, '.header-links .cart-qty', '.header-links .wishlist-qty', '#flyout-cart');
        
        


    
    
        
            
        
    
    
        
    
        
            ravireddy4qa@gmail.com
            Log out
                            
                
                    Shopping cart
                    (20)
                
            
                    
                
                    Wishlist
                    (0)
                
            
        
    
        
            $(document).ready(function () {
                $('.header').on('mouseenter', '#topcartlink', function () {
                    $('#flyout-cart').addClass('active');
                });
                $('.header').on('mouseleave', '#topcartlink', function () {
                    $('#flyout-cart').removeClass('active');
                });
                $('.header').on('mouseenter', '#flyout-cart', function () {
                    $('#flyout-cart').addClass('active');
                });
                $('.header').on('mouseleave', '#flyout-cart', function () {
                    $('#flyout-cart').removeClass('active');
                });
            });
        


        
    
        
There are 20 item(s) in your cart.        
            
                    
                            
                                
                                    
                                
                            
                        
                            
                                Blue Jeans
                            
                            Unit price: 1.00
                            Quantity: 20
                        
                    
            
            Sub-Total: 20.00
            
                    
                            
    


    
    
        
    
    
    
        $(document).ready(function() {
            $(&quot;#small-searchterms&quot;).focus(function() {
                if (this.value == 'Search store') {
                    this.value = '';
                }
            });

            $(&quot;#small-searchterms&quot;).blur(function() {
                if (this.value == '') {
                    this.value = 'Search store';
                }
            });
        });

        function check_small_search_form() {
            var search_terms = $(&quot;#small-searchterms&quot;);
            if (search_terms.val() == &quot;&quot; || search_terms.val() == &quot;Search store&quot;) {
                alert('Please enter some search keyword');
                search_terms.focus();
                return false;
            }
            return true;
        }
    
        
            
                $(function() {
                    $('#small-searchterms').autocomplete({
                            delay: 500,
                            minLength: 3,
                            source: '/catalog/searchtermautocomplete',
                            select: function(event, ui) {
                                $(&quot;#small-searchterms&quot;).val(ui.item.label);
                                setLocation(ui.item.producturl);
                                return false;
                            }
                        })
                        .data(&quot;ui-autocomplete&quot;)._renderItem = function(ul, item) {
                            var t = item.label;
                            //html encode
                            t = htmlEncode(t);
                            return $(&quot;&lt;li>&lt;/li>&quot;)
                                .data(&quot;item.autocomplete&quot;, item)
                                .append(&quot;&lt;a>&quot; + t + &quot;&lt;/a>&quot;)
                            .appendTo(ul);
                    };
                });
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    


        
            


    
    
        Books
        
                

    
    
        Computers
        
                
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
    



    $('li', '.top-menu').on('mouseenter', function () {
        $('a', $(this)).first().addClass('hover');
        if (!$(this).parent().hasClass('top-menu')) {
            var width = $(this).innerWidth();
            $('.sublist', $(this)).first().css('left', width + 15);
        }
        $('.sublist', $(this)).first().addClass('active');
        $('.top-menu-triangle', $(this)).addClass('active');
    });

    $('li', '.top-menu').on('mouseleave', function () {
        $('a', $(this)).first().removeClass('hover');
        $('.sublist', $(this)).first().removeClass('active');
        $('.top-menu-triangle', $(this)).removeClass('active');
    });



    
        
            

            Categories
        
    
    
        
    
        Books
        
                

    
    
        Computers
        
                
                     
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                     
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
        
    
    
        $('a', $('#mob-menu-button')).toggle(function() {
                $('.mob-top-menu').addClass('show');
            },
            function() {
                $('.mob-top-menu').removeClass('show');
            }
        );

        $(function($) {
            $('.mob-top-menu .expand').click(function() {
                var parent = $(this).parent();
                if (parent.hasClass('active')) {
                    $(&quot;.sublist:first&quot;, parent).hide(300);
                    parent.removeClass('active');
                } else {
                    $(&quot;.sublist:first&quot;, parent).show(300);
                    parent.addClass('active');
                }
            });
        });
    

        
        
        
        
            
            
        
        
            

    
    


    
        Checkout
    
    
        
            
                
                    1
                    Billing address
                
                
                    
                    
                        
    
        
            Select a billing address from your address book or enter a new address.
            
                
                        Ravikanth Edamakanti, 7737 River Drive, Atlanta, Georgia 30012, United States
                    New Address
                
            
        
    
    
        
            

    
        $(function () {
            $(&quot;#BillingNewAddress_CountryId&quot;).change(function () {
                var selectedItem = $(this).val();
                var ddlStates = $(&quot;#BillingNewAddress_StateProvinceId&quot;);
                var statesProgress = $(&quot;#states-loading-progress&quot;);
                statesProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;GET&quot;,
                    url: &quot;/country/getstatesbycountryid&quot;,
                    data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                    success: function (data) {
                        ddlStates.html('');
                        $.each(data, function (id, option) {
                            ddlStates.append($('&lt;option>&lt;/option>').val(option.id).html(option.name));
                        });
                        statesProgress.hide();
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert('Failed to retrieve states.');
                        statesProgress.hide();
                    }
                });
            });
        });
    


    
        First name:
            
        *
        
    
    
        Last name:
            
        *
        

    
    
        Email:
            
        *
        
    
        
            Company:
                
            
        
            
            Country:
                Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D'Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People's Republic of
Kuwait
Kyrgyzstan
Lao People's Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

            *
            
        

        
            State / province:
                Other (Non US)

            Wait...
            
        
            
            City:
                

*            
        
            
            Address 1:
                
*            
        
            
            Address 2:
                
            
        
            
            Zip / postal code:
                
*            
        
            
            Phone number:
                
*            
        
            
            Fax number:
                
            
        


            
        
    
    


                        
                    
                    
                    
                        Billing.init('#co-billing-form', 'https://demowebshop.tricentis.com/checkout/OpcSaveBilling/', false);
                        if ($(&quot;#billing-address-select&quot;).length > 0) {
                            Billing.newAddress(!$('#billing-address-select').val());
                        }
                    
                    
                        
                        Loading next step...
                    
                
            
                
                    
                        2
                        Shipping address
                    
                    
                        
                        
    
        
            
                Select a shipping address from your address book or enter a new address.
                
                    
                            Ravikanth Edamakanti, 7737 River Drive, Atlanta, Georgia 30012, United States
                        New Address
                    
                
            
        
        
            
                

    
        $(function () {
            $(&quot;#ShippingNewAddress_CountryId&quot;).change(function () {
                var selectedItem = $(this).val();
                var ddlStates = $(&quot;#ShippingNewAddress_StateProvinceId&quot;);
                var statesProgress = $(&quot;#states-loading-progress&quot;);
                statesProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;GET&quot;,
                    url: &quot;/country/getstatesbycountryid&quot;,
                    data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                    success: function (data) {
                        ddlStates.html('');
                        $.each(data, function (id, option) {
                            ddlStates.append($('&lt;option>&lt;/option>').val(option.id).html(option.name));
                        });
                        statesProgress.hide();
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert('Failed to retrieve states.');
                        statesProgress.hide();
                    }
                });
            });
        });
    


    
        First name:
            
        *
        
    
    
        Last name:
            
        *
        

    
    
        Email:
            
        *
        
    
        
            Company:
                
            
        
            
            Country:
                Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D'Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People's Republic of
Kuwait
Kyrgyzstan
Lao People's Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

            *
            
        

        
            State / province:
                Other (Non US)

            Wait...
            
        
            
            City:
                

*            
        
            
            Address 1:
                
*            
        
            
            Address 2:
                
            
        
            
            Zip / postal code:
                
*            
        
            
            Phone number:
                
*            
        
            
            Fax number:
                
            
        


                
            
        
        
    
        
            
                
                In-Store Pickup
            
            Pick up your items at the store (put your store address here)
        


                        
                        
                            Shipping.init('#co-shipping-form', 'https://demowebshop.tricentis.com/checkout/OpcSaveShipping/');
                            if ($(&quot;#shipping-address-select&quot;).length > 0) {
                                Shipping.newAddress(!$('#shipping-address-select').val());
                            }
                        
                        
                            
                                « Back
                            
                             Loading next step...
                        
                    
                
                
                    
                        3
                        Shipping method
                    
                    
                        
                        
    
    
            
                    
                        
                            
                            Ground (10.00)
                        
                            
                                Compared to other shipping methods, like by flight or over seas, ground shipping is carried out closer to the earth
                            
                    
                    
                        
                            
                            Next Day Air (40.00)
                        
                            
                                The one day air shipping
                            
                    
                    
                        
                            
                            2nd Day Air (20.00)
                        
                            
                                The two day air shipping
                            
                    
            
    
    


                        
                            ShippingMethod.init('#co-shipping-method-form', 'https://demowebshop.tricentis.com/checkout/OpcSaveShippingMethod/');
                        
                        
                            
                                « Back
                            
                            Loading next step...
                        
                        
                    
                
            
                
                    4
                    Payment method
                
                
                    
                    
    
    
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Cash On Delivery (COD) (7.00)
                            
                        
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Check / Money Order (5.00)
                            
                        
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Credit Card
                            
                        
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Purchase Order
                            
                        
                    
            
    
    


                    
                    
                        PaymentMethod.init('#co-payment-method-form', 'https://demowebshop.tricentis.com/checkout/OpcSavePaymentMethod/');
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    5
                    Payment information
                
                
                    
                    
    
    
        
            

    
        
            You will pay by COD
        
    


        
        
            

        
    
    


                    
                    
                        PaymentInfo.init('#co-payment-info-form', 'https://demowebshop.tricentis.com/checkout/OpcSavePaymentInfo/');
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    6
                    Confirm order
                
                
                    
    
    
        
        
    
    
    
        

    
        
        
            
                Billing Address
            
            
                Ravikanth Edamakanti
            
            
                Email: ravireddy4qa@gmail.com
            
                
                    Phone: 9998292827
                
                            
                    Fax: 
                
                                        
                    7737 River Drive
                
                                        
Atlanta                                            ,
                    Georgia                    30012                
                            
                    United States
                
                            
                        Payment Method
                
                
                        Cash On Delivery (COD)
                
        
            
                    
                        
                            Shipping Address
                    
                    
                        Ravikanth Edamakanti
                    
                    
                        Email: ravireddy4qa@gmail.com
                    
                        
                            Phone: 9998292827
                        
                        
                            Fax: 
                        
                        
                            7737 River Drive
                        
                        
Atlanta                                                            ,
                            Georgia                            30012                        
                        
                            United States
                        
                
                    Shipping Method
                
                
                    Ground
                
            
    

        
            
                                                    
                
                
                
                
            
            
                
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                                                                            
                                
                            
                        
                            Blue Jeans
                                                                                                            
                        
                            Price:
                            1.00
                        
                        
                            Qty.:
                                20
                        
                        
                            Total:
                            20.00
                        
                    
            
        
        
        
        
            
            
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    20.00 
                
            
            
                
                    
                        Shipping:
                        
                            (Ground)
                        
                
                
                    
                            10.00
                            
                    
                
            
                
                    
                        Payment method additional fee:
                    
                    
                        7.00
                        
                    
                
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            37.00
                    
                
            
        
    


            
        
    


        


                    
                        ConfirmOrder.init('https://demowebshop.tricentis.com/checkout/OpcConfirmOrder/', 'https://demowebshop.tricentis.com/checkout/completed/');
                    
                    
                        
                            « Back
                        
                        Submitting order information...
                    
                
            
        
    
    
        Accordion.init('checkout-steps', '.step-title', true);
        Accordion.openSection('#opc-billing');
        Checkout.init('https://demowebshop.tricentis.com/cart/');
        if (Billing.disableBillingAddressCheckoutStep)
        {
            Accordion.hideSection('#opc-billing');
            Billing.save();
        }
    


    


        
        
    
    

    
        
            Information
            
                    Sitemap
                Shipping &amp; Returns
                Privacy Notice
                Conditions of Use
                About us
                Contact us
            
        
        
            Customer service
            
                Search 
                    News
                                    Blog
                                                    Recently viewed products
                                    Compare products list
                                    New products
            
        
        
            My account
            
                My account
                    Orders
                                    Addresses
                                    Shopping cart
                                    Wishlist
            
        
        
            Follow us
            
                    Facebook
                                                    Twitter
                                                    RSS
                                                    YouTube
                                                    Google+
            
        
    
    
        Powered by nopCommerce
        
    
    
        Copyright © 2023 Tricentis Demo Web Shop. All rights reserved.
    
    
        


    
    





var _gaq = _gaq || [];
_gaq.push(['_setAccount', 'UA-6574346-11']);
_gaq.push(['_trackPageview']);

(function() {
    var ga = document.createElement('script'); ga.type = 'text/javascript'; ga.async = true;
    ga.src = ('https:' == document.location.protocol ? 'https://ssl' : 'http://www') + '.google-analytics.com/ga.js';
    var s = document.getElementsByTagName('script')[0]; s.parentNode.insertBefore(ga, s);
})();



    
    


/html[1]/body[1]</value>
      <webElementGuid>bf2428ad-de1f-42e6-822f-2cf223a57950</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]</value>
      <webElementGuid>4cdae53a-4aac-44ae-a08d-6c543eb2c25e</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>9905a509-2e3e-4702-b2e0-7e330efe0ee9</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
    







     





    
    
        
            AjaxCart.init(false, &quot; , &quot;'&quot; , &quot;.header-links .cart-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.header-links .wishlist-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;);
        
        


    
    
        
            
        
    
    
        
    
        
            ravireddy4qa@gmail.com
            Log out
                            
                
                    Shopping cart
                    (20)
                
            
                    
                
                    Wishlist
                    (0)
                
            
        
    
        
            $(document).ready(function () {
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
            });
        


        
    
        
There are 20 item(s) in your cart.        
            
                    
                            
                                
                                    
                                
                            
                        
                            
                                Blue Jeans
                            
                            Unit price: 1.00
                            Quantity: 20
                        
                    
            
            Sub-Total: 20.00
            
                    
                            
    


    
    
        
    
    
    
        $(document).ready(function() {
            $(&quot;#small-searchterms&quot;).focus(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }
            });

            $(&quot;#small-searchterms&quot;).blur(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;;
                }
            });
        });

        function check_small_search_form() {
            var search_terms = $(&quot;#small-searchterms&quot;);
            if (search_terms.val() == &quot;&quot; || search_terms.val() == &quot;Search store&quot;) {
                alert(&quot; , &quot;'&quot; , &quot;Please enter some search keyword&quot; , &quot;'&quot; , &quot;);
                search_terms.focus();
                return false;
            }
            return true;
        }
    
        
            
                $(function() {
                    $(&quot; , &quot;'&quot; , &quot;#small-searchterms&quot; , &quot;'&quot; , &quot;).autocomplete({
                            delay: 500,
                            minLength: 3,
                            source: &quot; , &quot;'&quot; , &quot;/catalog/searchtermautocomplete&quot; , &quot;'&quot; , &quot;,
                            select: function(event, ui) {
                                $(&quot;#small-searchterms&quot;).val(ui.item.label);
                                setLocation(ui.item.producturl);
                                return false;
                            }
                        })
                        .data(&quot;ui-autocomplete&quot;)._renderItem = function(ul, item) {
                            var t = item.label;
                            //html encode
                            t = htmlEncode(t);
                            return $(&quot;&lt;li>&lt;/li>&quot;)
                                .data(&quot;item.autocomplete&quot;, item)
                                .append(&quot;&lt;a>&quot; + t + &quot;&lt;/a>&quot;)
                            .appendTo(ul);
                    };
                });
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    


        
            


    
    
        Books
        
                

    
    
        Computers
        
                
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
    



    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        if (!$(this).parent().hasClass(&quot; , &quot;'&quot; , &quot;top-menu&quot; , &quot;'&quot; , &quot;)) {
            var width = $(this).innerWidth();
            $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().css(&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;, width + 15);
        }
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });

    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });



    
        
            

            Categories
        
    
    
        
    
        Books
        
                

    
    
        Computers
        
                
                     
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                     
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
        
    
    
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;#mob-menu-button&quot; , &quot;'&quot; , &quot;)).toggle(function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            },
            function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            }
        );

        $(function($) {
            $(&quot; , &quot;'&quot; , &quot;.mob-top-menu .expand&quot; , &quot;'&quot; , &quot;).click(function() {
                var parent = $(this).parent();
                if (parent.hasClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;)) {
                    $(&quot;.sublist:first&quot;, parent).hide(300);
                    parent.removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot;.sublist:first&quot;, parent).show(300);
                    parent.addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                }
            });
        });
    

        
        
        
        
            
            
        
        
            

    
    


    
        Checkout
    
    
        
            
                
                    1
                    Billing address
                
                
                    
                    
                        
    
        
            Select a billing address from your address book or enter a new address.
            
                
                        Ravikanth Edamakanti, 7737 River Drive, Atlanta, Georgia 30012, United States
                    New Address
                
            
        
    
    
        
            

    
        $(function () {
            $(&quot;#BillingNewAddress_CountryId&quot;).change(function () {
                var selectedItem = $(this).val();
                var ddlStates = $(&quot;#BillingNewAddress_StateProvinceId&quot;);
                var statesProgress = $(&quot;#states-loading-progress&quot;);
                statesProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;GET&quot;,
                    url: &quot;/country/getstatesbycountryid&quot;,
                    data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                    success: function (data) {
                        ddlStates.html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        $.each(data, function (id, option) {
                            ddlStates.append($(&quot; , &quot;'&quot; , &quot;&lt;option>&lt;/option>&quot; , &quot;'&quot; , &quot;).val(option.id).html(option.name));
                        });
                        statesProgress.hide();
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot; , &quot;'&quot; , &quot;Failed to retrieve states.&quot; , &quot;'&quot; , &quot;);
                        statesProgress.hide();
                    }
                });
            });
        });
    


    
        First name:
            
        *
        
    
    
        Last name:
            
        *
        

    
    
        Email:
            
        *
        
    
        
            Company:
                
            
        
            
            Country:
                Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D&quot; , &quot;'&quot; , &quot;Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of
Kuwait
Kyrgyzstan
Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

            *
            
        

        
            State / province:
                Other (Non US)

            Wait...
            
        
            
            City:
                

*            
        
            
            Address 1:
                
*            
        
            
            Address 2:
                
            
        
            
            Zip / postal code:
                
*            
        
            
            Phone number:
                
*            
        
            
            Fax number:
                
            
        


            
        
    
    


                        
                    
                    
                    
                        Billing.init(&quot; , &quot;'&quot; , &quot;#co-billing-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSaveBilling/&quot; , &quot;'&quot; , &quot;, false);
                        if ($(&quot;#billing-address-select&quot;).length > 0) {
                            Billing.newAddress(!$(&quot; , &quot;'&quot; , &quot;#billing-address-select&quot; , &quot;'&quot; , &quot;).val());
                        }
                    
                    
                        
                        Loading next step...
                    
                
            
                
                    
                        2
                        Shipping address
                    
                    
                        
                        
    
        
            
                Select a shipping address from your address book or enter a new address.
                
                    
                            Ravikanth Edamakanti, 7737 River Drive, Atlanta, Georgia 30012, United States
                        New Address
                    
                
            
        
        
            
                

    
        $(function () {
            $(&quot;#ShippingNewAddress_CountryId&quot;).change(function () {
                var selectedItem = $(this).val();
                var ddlStates = $(&quot;#ShippingNewAddress_StateProvinceId&quot;);
                var statesProgress = $(&quot;#states-loading-progress&quot;);
                statesProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;GET&quot;,
                    url: &quot;/country/getstatesbycountryid&quot;,
                    data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                    success: function (data) {
                        ddlStates.html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        $.each(data, function (id, option) {
                            ddlStates.append($(&quot; , &quot;'&quot; , &quot;&lt;option>&lt;/option>&quot; , &quot;'&quot; , &quot;).val(option.id).html(option.name));
                        });
                        statesProgress.hide();
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot; , &quot;'&quot; , &quot;Failed to retrieve states.&quot; , &quot;'&quot; , &quot;);
                        statesProgress.hide();
                    }
                });
            });
        });
    


    
        First name:
            
        *
        
    
    
        Last name:
            
        *
        

    
    
        Email:
            
        *
        
    
        
            Company:
                
            
        
            
            Country:
                Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D&quot; , &quot;'&quot; , &quot;Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of
Kuwait
Kyrgyzstan
Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

            *
            
        

        
            State / province:
                Other (Non US)

            Wait...
            
        
            
            City:
                

*            
        
            
            Address 1:
                
*            
        
            
            Address 2:
                
            
        
            
            Zip / postal code:
                
*            
        
            
            Phone number:
                
*            
        
            
            Fax number:
                
            
        


                
            
        
        
    
        
            
                
                In-Store Pickup
            
            Pick up your items at the store (put your store address here)
        


                        
                        
                            Shipping.init(&quot; , &quot;'&quot; , &quot;#co-shipping-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSaveShipping/&quot; , &quot;'&quot; , &quot;);
                            if ($(&quot;#shipping-address-select&quot;).length > 0) {
                                Shipping.newAddress(!$(&quot; , &quot;'&quot; , &quot;#shipping-address-select&quot; , &quot;'&quot; , &quot;).val());
                            }
                        
                        
                            
                                « Back
                            
                             Loading next step...
                        
                    
                
                
                    
                        3
                        Shipping method
                    
                    
                        
                        
    
    
            
                    
                        
                            
                            Ground (10.00)
                        
                            
                                Compared to other shipping methods, like by flight or over seas, ground shipping is carried out closer to the earth
                            
                    
                    
                        
                            
                            Next Day Air (40.00)
                        
                            
                                The one day air shipping
                            
                    
                    
                        
                            
                            2nd Day Air (20.00)
                        
                            
                                The two day air shipping
                            
                    
            
    
    


                        
                            ShippingMethod.init(&quot; , &quot;'&quot; , &quot;#co-shipping-method-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSaveShippingMethod/&quot; , &quot;'&quot; , &quot;);
                        
                        
                            
                                « Back
                            
                            Loading next step...
                        
                        
                    
                
            
                
                    4
                    Payment method
                
                
                    
                    
    
    
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Cash On Delivery (COD) (7.00)
                            
                        
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Check / Money Order (5.00)
                            
                        
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Credit Card
                            
                        
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Purchase Order
                            
                        
                    
            
    
    


                    
                    
                        PaymentMethod.init(&quot; , &quot;'&quot; , &quot;#co-payment-method-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSavePaymentMethod/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    5
                    Payment information
                
                
                    
                    
    
    
        
            

    
        
            You will pay by COD
        
    


        
        
            

        
    
    


                    
                    
                        PaymentInfo.init(&quot; , &quot;'&quot; , &quot;#co-payment-info-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSavePaymentInfo/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    6
                    Confirm order
                
                
                    
    
    
        
        
    
    
    
        

    
        
        
            
                Billing Address
            
            
                Ravikanth Edamakanti
            
            
                Email: ravireddy4qa@gmail.com
            
                
                    Phone: 9998292827
                
                            
                    Fax: 
                
                                        
                    7737 River Drive
                
                                        
Atlanta                                            ,
                    Georgia                    30012                
                            
                    United States
                
                            
                        Payment Method
                
                
                        Cash On Delivery (COD)
                
        
            
                    
                        
                            Shipping Address
                    
                    
                        Ravikanth Edamakanti
                    
                    
                        Email: ravireddy4qa@gmail.com
                    
                        
                            Phone: 9998292827
                        
                        
                            Fax: 
                        
                        
                            7737 River Drive
                        
                        
Atlanta                                                            ,
                            Georgia                            30012                        
                        
                            United States
                        
                
                    Shipping Method
                
                
                    Ground
                
            
    

        
            
                                                    
                
                
                
                
            
            
                
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                                                                            
                                
                            
                        
                            Blue Jeans
                                                                                                            
                        
                            Price:
                            1.00
                        
                        
                            Qty.:
                                20
                        
                        
                            Total:
                            20.00
                        
                    
            
        
        
        
        
            
            
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    20.00 
                
            
            
                
                    
                        Shipping:
                        
                            (Ground)
                        
                
                
                    
                            10.00
                            
                    
                
            
                
                    
                        Payment method additional fee:
                    
                    
                        7.00
                        
                    
                
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            37.00
                    
                
            
        
    


            
        
    


        


                    
                        ConfirmOrder.init(&quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcConfirmOrder/&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/completed/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Submitting order information...
                    
                
            
        
    
    
        Accordion.init(&quot; , &quot;'&quot; , &quot;checkout-steps&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.step-title&quot; , &quot;'&quot; , &quot;, true);
        Accordion.openSection(&quot; , &quot;'&quot; , &quot;#opc-billing&quot; , &quot;'&quot; , &quot;);
        Checkout.init(&quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/cart/&quot; , &quot;'&quot; , &quot;);
        if (Billing.disableBillingAddressCheckoutStep)
        {
            Accordion.hideSection(&quot; , &quot;'&quot; , &quot;#opc-billing&quot; , &quot;'&quot; , &quot;);
            Billing.save();
        }
    


    


        
        
    
    

    
        
            Information
            
                    Sitemap
                Shipping &amp; Returns
                Privacy Notice
                Conditions of Use
                About us
                Contact us
            
        
        
            Customer service
            
                Search 
                    News
                                    Blog
                                                    Recently viewed products
                                    Compare products list
                                    New products
            
        
        
            My account
            
                My account
                    Orders
                                    Addresses
                                    Shopping cart
                                    Wishlist
            
        
        
            Follow us
            
                    Facebook
                                                    Twitter
                                                    RSS
                                                    YouTube
                                                    Google+
            
        
    
    
        Powered by nopCommerce
        
    
    
        Copyright © 2023 Tricentis Demo Web Shop. All rights reserved.
    
    
        


    
    





var _gaq = _gaq || [];
_gaq.push([&quot; , &quot;'&quot; , &quot;_setAccount&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA-6574346-11&quot; , &quot;'&quot; , &quot;]);
_gaq.push([&quot; , &quot;'&quot; , &quot;_trackPageview&quot; , &quot;'&quot; , &quot;]);

(function() {
    var ga = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;); ga.type = &quot; , &quot;'&quot; , &quot;text/javascript&quot; , &quot;'&quot; , &quot;; ga.async = true;
    ga.src = (&quot; , &quot;'&quot; , &quot;https:&quot; , &quot;'&quot; , &quot; == document.location.protocol ? &quot; , &quot;'&quot; , &quot;https://ssl&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;http://www&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot;.google-analytics.com/ga.js&quot; , &quot;'&quot; , &quot;;
    var s = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;)[0]; s.parentNode.insertBefore(ga, s);
})();



    
    


/html[1]/body[1]&quot;) or . = concat(&quot;
    







     





    
    
        
            AjaxCart.init(false, &quot; , &quot;'&quot; , &quot;.header-links .cart-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.header-links .wishlist-qty&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;);
        
        


    
    
        
            
        
    
    
        
    
        
            ravireddy4qa@gmail.com
            Log out
                            
                
                    Shopping cart
                    (20)
                
            
                    
                
                    Wishlist
                    (0)
                
            
        
    
        
            $(document).ready(function () {
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#topcartlink&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
                $(&quot; , &quot;'&quot; , &quot;.header&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;, function () {
                    $(&quot; , &quot;'&quot; , &quot;#flyout-cart&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                });
            });
        


        
    
        
There are 20 item(s) in your cart.        
            
                    
                            
                                
                                    
                                
                            
                        
                            
                                Blue Jeans
                            
                            Unit price: 1.00
                            Quantity: 20
                        
                    
            
            Sub-Total: 20.00
            
                    
                            
    


    
    
        
    
    
    
        $(document).ready(function() {
            $(&quot;#small-searchterms&quot;).focus(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
                }
            });

            $(&quot;#small-searchterms&quot;).blur(function() {
                if (this.value == &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;) {
                    this.value = &quot; , &quot;'&quot; , &quot;Search store&quot; , &quot;'&quot; , &quot;;
                }
            });
        });

        function check_small_search_form() {
            var search_terms = $(&quot;#small-searchterms&quot;);
            if (search_terms.val() == &quot;&quot; || search_terms.val() == &quot;Search store&quot;) {
                alert(&quot; , &quot;'&quot; , &quot;Please enter some search keyword&quot; , &quot;'&quot; , &quot;);
                search_terms.focus();
                return false;
            }
            return true;
        }
    
        
            
                $(function() {
                    $(&quot; , &quot;'&quot; , &quot;#small-searchterms&quot; , &quot;'&quot; , &quot;).autocomplete({
                            delay: 500,
                            minLength: 3,
                            source: &quot; , &quot;'&quot; , &quot;/catalog/searchtermautocomplete&quot; , &quot;'&quot; , &quot;,
                            select: function(event, ui) {
                                $(&quot;#small-searchterms&quot;).val(ui.item.label);
                                setLocation(ui.item.producturl);
                                return false;
                            }
                        })
                        .data(&quot;ui-autocomplete&quot;)._renderItem = function(ul, item) {
                            var t = item.label;
                            //html encode
                            t = htmlEncode(t);
                            return $(&quot;&lt;li>&lt;/li>&quot;)
                                .data(&quot;item.autocomplete&quot;, item)
                                .append(&quot;&lt;a>&quot; + t + &quot;&lt;/a>&quot;)
                            .appendTo(ul);
                    };
                });
            
        

    
    
        
            
        
        
            
        
        
            
        
        
    


        
            


    
    
        Books
        
                

    
    
        Computers
        
                
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
    



    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseenter&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        if (!$(this).parent().hasClass(&quot; , &quot;'&quot; , &quot;top-menu&quot; , &quot;'&quot; , &quot;)) {
            var width = $(this).innerWidth();
            $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().css(&quot; , &quot;'&quot; , &quot;left&quot; , &quot;'&quot; , &quot;, width + 15);
        }
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });

    $(&quot; , &quot;'&quot; , &quot;li&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.top-menu&quot; , &quot;'&quot; , &quot;).on(&quot; , &quot;'&quot; , &quot;mouseleave&quot; , &quot;'&quot; , &quot;, function () {
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;hover&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.sublist&quot; , &quot;'&quot; , &quot;, $(this)).first().removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
        $(&quot; , &quot;'&quot; , &quot;.top-menu-triangle&quot; , &quot;'&quot; , &quot;, $(this)).removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
    });



    
        
            

            Categories
        
    
    
        
    
        Books
        
                

    
    
        Computers
        
                
                     
                
    
        Desktops
        

    
    
        Notebooks
        

    
    
        Accessories
        

    
                

    
    
        Electronics
        
                
                     
                
    
        Camera, photo
        

    
    
        Cell phones
        

    
                

    
    
        Apparel &amp; Shoes
        
                

    
    
        Digital downloads
        
                

    
    
        Jewelry
        
                

    
    
        Gift Cards
        
                

    
        
    
    
        $(&quot; , &quot;'&quot; , &quot;a&quot; , &quot;'&quot; , &quot;, $(&quot; , &quot;'&quot; , &quot;#mob-menu-button&quot; , &quot;'&quot; , &quot;)).toggle(function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).addClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            },
            function() {
                $(&quot; , &quot;'&quot; , &quot;.mob-top-menu&quot; , &quot;'&quot; , &quot;).removeClass(&quot; , &quot;'&quot; , &quot;show&quot; , &quot;'&quot; , &quot;);
            }
        );

        $(function($) {
            $(&quot; , &quot;'&quot; , &quot;.mob-top-menu .expand&quot; , &quot;'&quot; , &quot;).click(function() {
                var parent = $(this).parent();
                if (parent.hasClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;)) {
                    $(&quot;.sublist:first&quot;, parent).hide(300);
                    parent.removeClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                } else {
                    $(&quot;.sublist:first&quot;, parent).show(300);
                    parent.addClass(&quot; , &quot;'&quot; , &quot;active&quot; , &quot;'&quot; , &quot;);
                }
            });
        });
    

        
        
        
        
            
            
        
        
            

    
    


    
        Checkout
    
    
        
            
                
                    1
                    Billing address
                
                
                    
                    
                        
    
        
            Select a billing address from your address book or enter a new address.
            
                
                        Ravikanth Edamakanti, 7737 River Drive, Atlanta, Georgia 30012, United States
                    New Address
                
            
        
    
    
        
            

    
        $(function () {
            $(&quot;#BillingNewAddress_CountryId&quot;).change(function () {
                var selectedItem = $(this).val();
                var ddlStates = $(&quot;#BillingNewAddress_StateProvinceId&quot;);
                var statesProgress = $(&quot;#states-loading-progress&quot;);
                statesProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;GET&quot;,
                    url: &quot;/country/getstatesbycountryid&quot;,
                    data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                    success: function (data) {
                        ddlStates.html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        $.each(data, function (id, option) {
                            ddlStates.append($(&quot; , &quot;'&quot; , &quot;&lt;option>&lt;/option>&quot; , &quot;'&quot; , &quot;).val(option.id).html(option.name));
                        });
                        statesProgress.hide();
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot; , &quot;'&quot; , &quot;Failed to retrieve states.&quot; , &quot;'&quot; , &quot;);
                        statesProgress.hide();
                    }
                });
            });
        });
    


    
        First name:
            
        *
        
    
    
        Last name:
            
        *
        

    
    
        Email:
            
        *
        
    
        
            Company:
                
            
        
            
            Country:
                Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D&quot; , &quot;'&quot; , &quot;Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of
Kuwait
Kyrgyzstan
Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

            *
            
        

        
            State / province:
                Other (Non US)

            Wait...
            
        
            
            City:
                

*            
        
            
            Address 1:
                
*            
        
            
            Address 2:
                
            
        
            
            Zip / postal code:
                
*            
        
            
            Phone number:
                
*            
        
            
            Fax number:
                
            
        


            
        
    
    


                        
                    
                    
                    
                        Billing.init(&quot; , &quot;'&quot; , &quot;#co-billing-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSaveBilling/&quot; , &quot;'&quot; , &quot;, false);
                        if ($(&quot;#billing-address-select&quot;).length > 0) {
                            Billing.newAddress(!$(&quot; , &quot;'&quot; , &quot;#billing-address-select&quot; , &quot;'&quot; , &quot;).val());
                        }
                    
                    
                        
                        Loading next step...
                    
                
            
                
                    
                        2
                        Shipping address
                    
                    
                        
                        
    
        
            
                Select a shipping address from your address book or enter a new address.
                
                    
                            Ravikanth Edamakanti, 7737 River Drive, Atlanta, Georgia 30012, United States
                        New Address
                    
                
            
        
        
            
                

    
        $(function () {
            $(&quot;#ShippingNewAddress_CountryId&quot;).change(function () {
                var selectedItem = $(this).val();
                var ddlStates = $(&quot;#ShippingNewAddress_StateProvinceId&quot;);
                var statesProgress = $(&quot;#states-loading-progress&quot;);
                statesProgress.show();
                $.ajax({
                    cache: false,
                    type: &quot;GET&quot;,
                    url: &quot;/country/getstatesbycountryid&quot;,
                    data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                    success: function (data) {
                        ddlStates.html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                        $.each(data, function (id, option) {
                            ddlStates.append($(&quot; , &quot;'&quot; , &quot;&lt;option>&lt;/option>&quot; , &quot;'&quot; , &quot;).val(option.id).html(option.name));
                        });
                        statesProgress.hide();
                    },
                    error: function (xhr, ajaxOptions, thrownError) {
                        alert(&quot; , &quot;'&quot; , &quot;Failed to retrieve states.&quot; , &quot;'&quot; , &quot;);
                        statesProgress.hide();
                    }
                });
            });
        });
    


    
        First name:
            
        *
        
    
    
        Last name:
            
        *
        

    
    
        Email:
            
        *
        
    
        
            Company:
                
            
        
            
            Country:
                Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D&quot; , &quot;'&quot; , &quot;Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of
Kuwait
Kyrgyzstan
Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

            *
            
        

        
            State / province:
                Other (Non US)

            Wait...
            
        
            
            City:
                

*            
        
            
            Address 1:
                
*            
        
            
            Address 2:
                
            
        
            
            Zip / postal code:
                
*            
        
            
            Phone number:
                
*            
        
            
            Fax number:
                
            
        


                
            
        
        
    
        
            
                
                In-Store Pickup
            
            Pick up your items at the store (put your store address here)
        


                        
                        
                            Shipping.init(&quot; , &quot;'&quot; , &quot;#co-shipping-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSaveShipping/&quot; , &quot;'&quot; , &quot;);
                            if ($(&quot;#shipping-address-select&quot;).length > 0) {
                                Shipping.newAddress(!$(&quot; , &quot;'&quot; , &quot;#shipping-address-select&quot; , &quot;'&quot; , &quot;).val());
                            }
                        
                        
                            
                                « Back
                            
                             Loading next step...
                        
                    
                
                
                    
                        3
                        Shipping method
                    
                    
                        
                        
    
    
            
                    
                        
                            
                            Ground (10.00)
                        
                            
                                Compared to other shipping methods, like by flight or over seas, ground shipping is carried out closer to the earth
                            
                    
                    
                        
                            
                            Next Day Air (40.00)
                        
                            
                                The one day air shipping
                            
                    
                    
                        
                            
                            2nd Day Air (20.00)
                        
                            
                                The two day air shipping
                            
                    
            
    
    


                        
                            ShippingMethod.init(&quot; , &quot;'&quot; , &quot;#co-shipping-method-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSaveShippingMethod/&quot; , &quot;'&quot; , &quot;);
                        
                        
                            
                                « Back
                            
                            Loading next step...
                        
                        
                    
                
            
                
                    4
                    Payment method
                
                
                    
                    
    
    
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Cash On Delivery (COD) (7.00)
                            
                        
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Check / Money Order (5.00)
                            
                        
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Credit Card
                            
                        
                    
                    
                        
                                
                                    
                                        
                                    
                                
                            
                                
                                Purchase Order
                            
                        
                    
            
    
    


                    
                    
                        PaymentMethod.init(&quot; , &quot;'&quot; , &quot;#co-payment-method-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSavePaymentMethod/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    5
                    Payment information
                
                
                    
                    
    
    
        
            

    
        
            You will pay by COD
        
    


        
        
            

        
    
    


                    
                    
                        PaymentInfo.init(&quot; , &quot;'&quot; , &quot;#co-payment-info-form&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcSavePaymentInfo/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Loading next step...
                    
                
            
            
                
                    6
                    Confirm order
                
                
                    
    
    
        
        
    
    
    
        

    
        
        
            
                Billing Address
            
            
                Ravikanth Edamakanti
            
            
                Email: ravireddy4qa@gmail.com
            
                
                    Phone: 9998292827
                
                            
                    Fax: 
                
                                        
                    7737 River Drive
                
                                        
Atlanta                                            ,
                    Georgia                    30012                
                            
                    United States
                
                            
                        Payment Method
                
                
                        Cash On Delivery (COD)
                
        
            
                    
                        
                            Shipping Address
                    
                    
                        Ravikanth Edamakanti
                    
                    
                        Email: ravireddy4qa@gmail.com
                    
                        
                            Phone: 9998292827
                        
                        
                            Fax: 
                        
                        
                            7737 River Drive
                        
                        
Atlanta                                                            ,
                            Georgia                            30012                        
                        
                            United States
                        
                
                    Shipping Method
                
                
                    Ground
                
            
    

        
            
                                                    
                
                
                
                
            
            
                
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                                                                            
                                
                            
                        
                            Blue Jeans
                                                                                                            
                        
                            Price:
                            1.00
                        
                        
                            Qty.:
                                20
                        
                        
                            Total:
                            20.00
                        
                    
            
        
        
        
        
            
            
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    20.00 
                
            
            
                
                    
                        Shipping:
                        
                            (Ground)
                        
                
                
                    
                            10.00
                            
                    
                
            
                
                    
                        Payment method additional fee:
                    
                    
                        7.00
                        
                    
                
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            37.00
                    
                
            
        
    


            
        
    


        


                    
                        ConfirmOrder.init(&quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/OpcConfirmOrder/&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/checkout/completed/&quot; , &quot;'&quot; , &quot;);
                    
                    
                        
                            « Back
                        
                        Submitting order information...
                    
                
            
        
    
    
        Accordion.init(&quot; , &quot;'&quot; , &quot;checkout-steps&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;.step-title&quot; , &quot;'&quot; , &quot;, true);
        Accordion.openSection(&quot; , &quot;'&quot; , &quot;#opc-billing&quot; , &quot;'&quot; , &quot;);
        Checkout.init(&quot; , &quot;'&quot; , &quot;https://demowebshop.tricentis.com/cart/&quot; , &quot;'&quot; , &quot;);
        if (Billing.disableBillingAddressCheckoutStep)
        {
            Accordion.hideSection(&quot; , &quot;'&quot; , &quot;#opc-billing&quot; , &quot;'&quot; , &quot;);
            Billing.save();
        }
    


    


        
        
    
    

    
        
            Information
            
                    Sitemap
                Shipping &amp; Returns
                Privacy Notice
                Conditions of Use
                About us
                Contact us
            
        
        
            Customer service
            
                Search 
                    News
                                    Blog
                                                    Recently viewed products
                                    Compare products list
                                    New products
            
        
        
            My account
            
                My account
                    Orders
                                    Addresses
                                    Shopping cart
                                    Wishlist
            
        
        
            Follow us
            
                    Facebook
                                                    Twitter
                                                    RSS
                                                    YouTube
                                                    Google+
            
        
    
    
        Powered by nopCommerce
        
    
    
        Copyright © 2023 Tricentis Demo Web Shop. All rights reserved.
    
    
        


    
    





var _gaq = _gaq || [];
_gaq.push([&quot; , &quot;'&quot; , &quot;_setAccount&quot; , &quot;'&quot; , &quot;, &quot; , &quot;'&quot; , &quot;UA-6574346-11&quot; , &quot;'&quot; , &quot;]);
_gaq.push([&quot; , &quot;'&quot; , &quot;_trackPageview&quot; , &quot;'&quot; , &quot;]);

(function() {
    var ga = document.createElement(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;); ga.type = &quot; , &quot;'&quot; , &quot;text/javascript&quot; , &quot;'&quot; , &quot;; ga.async = true;
    ga.src = (&quot; , &quot;'&quot; , &quot;https:&quot; , &quot;'&quot; , &quot; == document.location.protocol ? &quot; , &quot;'&quot; , &quot;https://ssl&quot; , &quot;'&quot; , &quot; : &quot; , &quot;'&quot; , &quot;http://www&quot; , &quot;'&quot; , &quot;) + &quot; , &quot;'&quot; , &quot;.google-analytics.com/ga.js&quot; , &quot;'&quot; , &quot;;
    var s = document.getElementsByTagName(&quot; , &quot;'&quot; , &quot;script&quot; , &quot;'&quot; , &quot;)[0]; s.parentNode.insertBefore(ga, s);
})();



    
    


/html[1]/body[1]&quot;))]</value>
      <webElementGuid>51c33b36-0e75-442d-99f2-031d6758e9c4</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
