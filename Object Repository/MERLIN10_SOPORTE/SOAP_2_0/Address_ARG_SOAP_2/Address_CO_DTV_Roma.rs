<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Address_CO_DTV_Roma</name>
   <tag></tag>
   <elementGuidId>7d00481f-9ffb-4f36-9a68-c92a4a820a32</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:soap=&quot;http://soap2.addressonline.ases.com/&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;soap:addressNormalize2>
         &lt;!--Optional:-->
         &lt;addressNormalize>
            &lt;!--Optional:-->
            &lt;clientAccessCode>1d39c2606f8fe4551604069ecada27fe&lt;/clientAccessCode>
            &lt;!--Optional:-->
            &lt;customAdaptersKeys>
               &lt;!--Zero or more repetitions:-->
               &lt;customAdapter>&lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xAddress>
               &lt;!--Optional:-->
               &lt;level1>&lt;/level1>
               &lt;!--Optional:-->
               &lt;level2>buenos aires&lt;/level2>
               &lt;!--Optional:-->
               &lt;level3>&lt;/level3>
               &lt;!--Optional:-->
               &lt;level4>monte chingolo&lt;/level4>
               &lt;!--Optional:-->
               &lt;level5>&lt;/level5>
               &lt;!--Optional:-->
               &lt;street>roma 4128&lt;/street>
               &lt;!--Optional:-->
               &lt;houseNumber> &lt;/houseNumber>
               &lt;!--Optional:-->
               &lt;floor>&lt;/floor>
               &lt;!--Optional:-->
               &lt;unit>&lt;/unit>
               &lt;!--Optional:-->
               &lt;postalCode>1825&lt;/postalCode>
               &lt;!--Optional:-->
               &lt;additionalData>&lt;/additionalData>
               &lt;!--Optional:-->
               &lt;betweenStreet1>&lt;/betweenStreet1>
               &lt;!--Optional:-->
               &lt;betweenStreet2>&lt;/betweenStreet2>
            &lt;/xAddress>
         &lt;/addressNormalize>
      &lt;/soap:addressNormalize2>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>addressNormalize2</soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.Address_ARG2</defaultValue>
      <description></description>
      <id>4d59abac-efa3-41b7-bd77-7640915b76d1</id>
      <masked>false</masked>
      <name>Address_ARG2</name>
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

/**verificacion test case**/

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'LANUS')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'MONTE CHINGOLO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'VILLA INDEPENDENCIA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'ROMA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '4128')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-34.741363')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-58.371764')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1825')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'B1825BPB')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '3401')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '4800')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'GRANADEROS')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'CORVALAN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.357121&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>4&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>tipoVivienda&lt;/name>&lt;value>otro&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.728784&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value>&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>SI&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>empresa&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')

/**verificacion WS unitaria

WS.verifyElementText(response, 'addressNormalize2Response.return.status', 'CO')
WS.verifyElementText(response, 'addressNormalize2Response.return.statusReason', 'SM')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.geoType', '1')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level1', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level2', 'BUENOS AIRES')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level3', 'LANUS')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level4', 'MONTE CHINGOLO')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.level5', 'VILLA INDEPENDENCIA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.street', 'ROMA')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.houseNumber', '4128')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.floor', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.unit', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.latitude', '-34.741363')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.longitude', '-58.371764')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.postalCode', '1825')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.additionalPostalCode', 'B1825BPB')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.fromStreetNumber', '3401')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.toStreetNumber', '4800')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet1', 'GRANADEROS')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.betweenStreet2', 'CORVALAN')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.corner', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeType', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.place', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.placeReference', '')
WS.verifyElementText(response, 'addressNormalize2Response.return.nAddress.numberAlternativeAddresses', '0')
assertThat(response.getResponseText()).contains('&lt;name>level4Longitude&lt;/name>&lt;value>-58.357121&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>nise&lt;/name>&lt;value>4&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>tipoVivienda&lt;/name>&lt;value>otro&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>level4Latitude&lt;/name>&lt;value>-34.728784&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqStatus&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>maqValue&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>maqConcept&lt;/name>&lt;value/>')
assertThat(response.getResponseText()).contains('&lt;name>postalCertifiedAddresses&lt;/name>&lt;value>SI&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>empresa&lt;/name>&lt;value>NO&lt;/value>')
assertThat(response.getResponseText()).contains('&lt;name>merlinRiskArea&lt;/name>&lt;value>N&lt;/value>')
**/
</verificationScript>
   <wsdlAddress>${Address_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
