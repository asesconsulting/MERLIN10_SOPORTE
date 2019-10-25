<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PersonSearch_EN_Nombre1</name>
   <tag></tag>
   <elementGuidId>d5ff2d7d-6d7a-497a-9347-4f7fd3645149</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:v2=&quot;http://v2.soap2.filiationonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;v2:personSearch>
         &lt;!--Optional:-->
         &lt;personSearch>
            &lt;!--Optional:-->
            &lt;clientAccessCode>a1edeae2a5bd4cde241fdfdb193ca13c&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter>&lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xPerson>
               &lt;!--Optional:-->
               &lt;documentType>&lt;/documentType>
               &lt;!--Optional:-->
               &lt;documentNumber>&lt;/documentNumber>
               &lt;!--Optional:-->
               &lt;tributaryType>&lt;/tributaryType>
               &lt;!--Optional:-->
               &lt;tributaryNumber>&lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName>bringas&lt;/lastName>
               &lt;!--Optional:-->
               &lt;name>mauro&lt;/name>
               &lt;!--Optional:-->
               &lt;gender>&lt;/gender>
               &lt;!--Optional:-->
               &lt;level2>&lt;/level2>
               &lt;!--Optional:-->
               &lt;level4>zarate&lt;/level4>
               &lt;!--Optional:-->
               &lt;postalCode>&lt;/postalCode>
            &lt;/xPerson>
         &lt;/personSearch>
      &lt;/v2:personSearch>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>personSearch</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.PersonSearch_ARG2</defaultValue>
      <description></description>
      <id>9370d00a-e1a9-4e8f-ad06-129e1cf51603</id>
      <masked>false</masked>
      <name>PersonSearch_ARG2</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyElementText(response, 'personSearchResponse.return.status', 'EN')
WS.verifyElementText(response, 'personSearchResponse.return.statusReason', 'SM')
WS.verifyElementText(response, 'personSearchResponse.return.nPerson.numberAlternativePerson', '15')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>25661412&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20256614120&lt;/tributaryNumber>&lt;name>MAURO RAMON&lt;/name>&lt;lastName>BRINGAS&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>07/02/1977&lt;/birthDate>&lt;street>ITUZAINGO&lt;/street>&lt;houseNumber>2636&lt;/houseNumber>&lt;floor> &lt;/floor>&lt;unit> &lt;/unit>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level4>ZARATE&lt;/level4>&lt;postalCode>2800&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>41441413&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20414414134&lt;/tributaryNumber>&lt;name>FACUNDO&lt;/name>&lt;lastName>BRINGAS&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>08/11/1998&lt;/birthDate>&lt;street>FLORESTANO ANDRADE&lt;/street>&lt;houseNumber>1432&lt;/houseNumber>&lt;floor> &lt;/floor>&lt;unit> &lt;/unit>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level4>ZARATE&lt;/level4>&lt;postalCode>2800&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>36981816&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20369818164&lt;/tributaryNumber>&lt;name>MAURO&lt;/name>&lt;lastName>BRINGAS&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>18/01/1993&lt;/birthDate>&lt;street>GOROSITO&lt;/street>&lt;houseNumber>12&lt;/houseNumber>&lt;floor> &lt;/floor>&lt;unit> &lt;/unit>&lt;level2>BUENOS AIRES&lt;/level2>&lt;level4>BELLA VISTA&lt;/level4>&lt;postalCode>1661&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>31347880&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20313478808&lt;/tributaryNumber>&lt;name>MAURO ALEJANDRO&lt;/name>&lt;lastName>BRINGAS&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>05/06/1985&lt;/birthDate>&lt;street>AVDA SANTA FE&lt;/street>&lt;houseNumber>3858&lt;/houseNumber>&lt;floor>2&lt;/floor>&lt;unit> &lt;/unit>&lt;level2>CAPITAL FEDERAL&lt;/level2>&lt;level4>CIUDAD AUTONOMA BUENOS AIRES&lt;/level4>&lt;postalCode>1425&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>47711748&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20477117482&lt;/tributaryNumber>&lt;name>MAURO SALVADOR&lt;/name>&lt;lastName>BRINGAS&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>24/05/2007&lt;/birthDate>&lt;street>&lt;/street>&lt;houseNumber>&lt;/houseNumber>&lt;floor>&lt;/floor>&lt;unit>&lt;/unit>&lt;level2>&lt;/level2>&lt;level4>&lt;/level4>&lt;postalCode>&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>44703730&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20447037301&lt;/tributaryNumber>&lt;name>MAURO SEBASTIAN&lt;/name>&lt;lastName>BRINGAS&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>09/03/2003&lt;/birthDate>&lt;street>FRANCISCO NARCISO DE LAPRIDA&lt;/street>&lt;houseNumber>580&lt;/houseNumber>&lt;floor>3&lt;/floor>&lt;unit>A&lt;/unit>&lt;level2>TUCUMAN&lt;/level2>&lt;level4>SAN MIGUEL DE TUCUMAN&lt;/level4>&lt;postalCode>4000&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')</verificationScript>
   <wsdlAddress>${PersonSearch_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
