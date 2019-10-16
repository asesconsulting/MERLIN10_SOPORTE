<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PersonSearch_EN</name>
   <tag></tag>
   <elementGuidId>090d9879-9306-4eac-b8e9-b6367ae5ceb6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:v2=&quot;http://v2.soap2.enrichment.ases.com/&quot;>
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
               &lt;customAdapter> &lt;/customAdapter>
            &lt;/customAdaptersKeys>
            &lt;!--Optional:-->
            &lt;xPerson>
               &lt;!--Optional:-->
               &lt;documentType> &lt;/documentType>
               &lt;!--Optional:-->
               &lt;documentNumber> &lt;/documentNumber>
               &lt;!--Optional:-->
               &lt;tributaryType> &lt;/tributaryType>
               &lt;!--Optional:-->
               &lt;tributaryNumber> &lt;/tributaryNumber>
               &lt;!--Optional:-->
               &lt;lastName>perez&lt;/lastName>
               &lt;!--Optional:-->
               &lt;name>juan&lt;/name>
               &lt;!--Optional:-->
               &lt;gender> &lt;/gender>
               &lt;!--Optional:-->
               &lt;level2> &lt;/level2>
               &lt;!--Optional:-->
               &lt;level4>cordoba&lt;/level4>
               &lt;!--Optional:-->
               &lt;postalCode> &lt;/postalCode>
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
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>6479718&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20064797183&lt;/tributaryNumber>&lt;name>JUAN&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>01/01/1932&lt;/birthDate>&lt;street>TOLOSA&lt;/street>&lt;houseNumber>2364&lt;/houseNumber>&lt;floor> &lt;/floor>&lt;unit> &lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5014&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>6458787&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20064587871&lt;/tributaryNumber>&lt;name>JUAN&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>07/01/1928&lt;/birthDate>&lt;street>BV SAN JUAN&lt;/street>&lt;houseNumber>345&lt;/houseNumber>&lt;floor>&lt;/floor>&lt;unit>&lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5000&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>2789519&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20027895191&lt;/tributaryNumber>&lt;name>JUAN&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>01/01/1922&lt;/birthDate>&lt;street>LA RIOJA&lt;/street>&lt;houseNumber>1976&lt;/houseNumber>&lt;floor>&lt;/floor>&lt;unit>&lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5003&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>2789519&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20027895191&lt;/tributaryNumber>&lt;name>JUAN&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>01/01/1922&lt;/birthDate>&lt;street>LA RIOJA&lt;/street>&lt;houseNumber>1976&lt;/houseNumber>&lt;floor>&lt;/floor>&lt;unit>&lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5003&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>90&lt;/documentType>&lt;documentNumber>6476035&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20064760352&lt;/tributaryNumber>&lt;name>JUAN&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>24/06/1930&lt;/birthDate>&lt;street>CALLE 26&lt;/street>&lt;houseNumber>&lt;/houseNumber>&lt;floor>&lt;/floor>&lt;unit>&lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5000&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>11194546&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20111945463&lt;/tributaryNumber>&lt;name>JUAN&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>14/02/1955&lt;/birthDate>&lt;street>ISLA VERDE&lt;/street>&lt;houseNumber>1584&lt;/houseNumber>&lt;floor> &lt;/floor>&lt;unit> &lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5017&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>6197776&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20061977768&lt;/tributaryNumber>&lt;name>JUAN&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>10/12/1927&lt;/birthDate>&lt;street>SALAMANCA&lt;/street>&lt;houseNumber>2728&lt;/houseNumber>&lt;floor>PB&lt;/floor>&lt;unit>3&lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5014&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>90&lt;/documentType>&lt;documentNumber>8226454&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>20082264540&lt;/tributaryNumber>&lt;name>JUAN&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>19/10/1944&lt;/birthDate>&lt;street>GRAL JUAN BAUTISTA BUSTOS&lt;/street>&lt;houseNumber>246&lt;/houseNumber>&lt;floor> &lt;/floor>&lt;unit> &lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5000&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>11054400&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>20110544007&lt;/tributaryNumber>&lt;name>JUAN CARLOS&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>26/02/1952&lt;/birthDate>&lt;street>PJE SANTO TOMAS 472&lt;/street>&lt;houseNumber>&lt;/houseNumber>&lt;floor>&lt;/floor>&lt;unit>&lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5000&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>11050257&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20110502576&lt;/tributaryNumber>&lt;name>JUAN CARLOS&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>18/10/1953&lt;/birthDate>&lt;street>FRANCFORT&lt;/street>&lt;houseNumber>1760&lt;/houseNumber>&lt;floor> &lt;/floor>&lt;unit> &lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5012&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>14703263&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>20147032634&lt;/tributaryNumber>&lt;name>OSCAR JUAN&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>24/06/1961&lt;/birthDate>&lt;street>ENRIQUE GRANADOS&lt;/street>&lt;houseNumber>1224&lt;/houseNumber>&lt;floor>PB&lt;/floor>&lt;unit>L 5&lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5010&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>90&lt;/documentType>&lt;documentNumber>6519424&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>20065194245&lt;/tributaryNumber>&lt;name>JUAN CARLOS&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>20/05/1941&lt;/birthDate>&lt;street>SANTIAGO CACERES&lt;/street>&lt;houseNumber>2839&lt;/houseNumber>&lt;floor> &lt;/floor>&lt;unit> &lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5016&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>6695320&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>20066953204&lt;/tributaryNumber>&lt;name>JUAN FRANCISCO&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>21/08/1944&lt;/birthDate>&lt;street>BALCARCE&lt;/street>&lt;houseNumber>470&lt;/houseNumber>&lt;floor>&lt;/floor>&lt;unit>&lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5000&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>96&lt;/documentType>&lt;documentNumber>6767702&lt;/documentNumber>&lt;tributaryType>86&lt;/tributaryType>&lt;tributaryNumber>20067677022&lt;/tributaryNumber>&lt;name>JUAN CARLOS&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>23/08/1939&lt;/birthDate>&lt;street>BV LAS HERAS&lt;/street>&lt;houseNumber>156&lt;/houseNumber>&lt;floor> &lt;/floor>&lt;unit> &lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5000&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')
assertThat(response.getResponseText()).contains('&lt;alternativePerson>&lt;documentType>90&lt;/documentType>&lt;documentNumber>7378199&lt;/documentNumber>&lt;tributaryType>80&lt;/tributaryType>&lt;tributaryNumber>20073781990&lt;/tributaryNumber>&lt;name>JUAN CARLOS&lt;/name>&lt;lastName>PEREZ&lt;/lastName>&lt;gender>M&lt;/gender>&lt;birthDate>19/06/1949&lt;/birthDate>&lt;street>SAN JERONIMO&lt;/street>&lt;houseNumber>630&lt;/houseNumber>&lt;floor>&lt;/floor>&lt;unit>&lt;/unit>&lt;level2>CORDOBA&lt;/level2>&lt;level4>CORDOBA&lt;/level4>&lt;postalCode>5000&lt;/postalCode>&lt;merlinCustomValues/>&lt;/alternativePerson>')</verificationScript>
   <wsdlAddress>${PersonSearch_ARG2}</wsdlAddress>
</WebServiceRequestEntity>
